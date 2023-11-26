use super::models::{AppData, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo};
use super::requests::{delete_cached_record, get_cached_record};
use super::watched::Watched;
use crate::common::models::DeleterrError;
use crate::overseerr::models::{AllSeasons, MediaInfo, MediaRequest, MediaType};
use crate::tautulli::user_watch_history::{GetAllOrNone, GetFirstOrNone, UserWatchHistory};
use actix_web::web::Data;
use std::collections::HashMap;

pub async fn get_request_status(
    rating_key: &Option<u64>,
    user_id: &Option<u64>,
    media_type: &MediaType,
    media_info: MediaInfo,
    media_request: &MediaRequest,
    all_seasons: AllSeasons,
) -> Result<RequestStatus, DeleterrError> {
    let tau_history_response = match (rating_key, user_id) {
        (Some(rk), Some(uid)) => Some(
            crate::tautulli::services::get_item_history(
                rk.to_string().as_str(),
                uid.to_string().as_str(),
                media_type,
            )
            .await?
            .response
            .data,
        ),
        _ => None,
    };

    let watched_status = match media_type {
        MediaType::Movie => {
            let tau_history = tau_history_response.get_first_or_none();
            let watched = Watched {
                req_parent_index_id: media_request.id,
                parent_media_index: None,
                req_status: media_request.status,
                total_items: Some(1),
                watched: tau_history.is_some(),
                watch_history: Vec::from([tau_history]),
            };
            Vec::from([watched])
        }
        MediaType::TV => {
            let tau_history = tau_history_response.get_all_or_none();
            let mut watched: Vec<Watched> = vec![];
            if let Some(history) = tau_history {
                for season in &media_request.seasons {
                    let season_watched: Vec<Option<UserWatchHistory>> = history
                        .clone()
                        .into_iter()
                        .filter(|item| item.parent_media_index == Some(season.season_number))
                        .map(Some)
                        .collect();

                    let total_items = all_seasons
                        .seasons
                        .iter()
                        .find(|all_seasons_season| {
                            season.season_number == all_seasons_season.season_number
                        })
                        .map(|matching_season| matching_season.episode_count);

                    let season_watched = Watched {
                        req_parent_index_id: season.id,
                        parent_media_index: Some(season.season_number),
                        req_status: season.status,
                        total_items: total_items,
                        watched: false,
                        watch_history: season_watched,
                    };

                    watched.push(season_watched)
                }
            }
            watched
        }
    };
    let request_status = RequestStatus {
        media_info,
        watched_status,
        media_request: media_request.clone(),
    };

    Ok(request_status)
}

pub async fn match_requests_to_watched(
    chunk: Option<usize>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let chunk_size = chunk.unwrap_or(10);
    // ! Note that the default take is 10 at overseerr if unspecified!
    let (os_requests, page_info) = crate::overseerr::services::get_os_requests().await?;
    let mut matched_requests: HashMap<usize, RequestStatus> =
        HashMap::with_capacity(page_info.results);

    for i in 0..os_requests.len() {
        let media_request = &os_requests[i];
        let (media_type, tmdb_id, rating_key, user_id) = (
            &media_request.media.media_type,
            &media_request.media.tmdb_id,
            &media_request.media.rating_key,
            &media_request.requested_by.plex_id,
        );

        let media_info = crate::overseerr::services::get_media_info(&media_type, &tmdb_id).await?;

        let all_seasons = match (media_type, media_request.media.tmdb_id) {
            (MediaType::TV, Some(tvid)) => crate::overseerr::services::get_seasons(tvid).await?,
            _ => AllSeasons::movie_season(),
        };

        let request_status = get_request_status(
            rating_key,
            user_id,
            media_type,
            media_info,
            media_request,
            all_seasons,
        )
        .await?;

        // Delay
        if i > 0 && (i % chunk_size) == 0 {
            println!("Sleeping for a sec");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }

        matched_requests.insert(request_status.media_request.id, request_status);
    }

    let request_status_with_record_info = RequestStatusWithRecordInfo {
        all_requests: page_info.results,
        requests: matched_requests,
    };

    Ok(request_status_with_record_info)
}

pub async fn delete_movie_from_radarr_overseerr(
    app_data: &Data<AppData>,
    request_id: usize,
) -> Result<MovieDeletionRequest, DeleterrError> {
    // Get the record from the cache
    let request = get_cached_record(app_data, request_id).ok_or(DeleterrError::new(
        "No request found! You may need to resync the APIs",
    ))?;

    let media_id = request
        .media_request
        .media
        .id
        .ok_or(DeleterrError::new("Missing media Id!"))?;

    let radarr_id = request
        .media_request
        .media
        .external_service_id
        .ok_or(DeleterrError::new("Missing external service (radarr) id!"))?;

    let radarr_response = crate::radarr::services::delete_movie(radarr_id.to_string().as_str())
        .await
        .map_err(|err| err.add_prefix("Unause crate::tautulli::services::get_item_history;ble to delete from Radarr. Error: "))?;

    let overseerr_response = crate::overseerr::services::delete_media(media_id.to_string().as_str())
        .await
        .map_err(|err| err.add_prefix("Radarr media deleted but was unable to delete Overseer request. Please delete it manually. Error: "))?;

    let cache_response = delete_cached_record(app_data, request_id)
        .map_err(|err| err.add_prefix("Deleted in both Radarr and Overseer but was unable to remove from Cache. Manually trigger a sync to fix the issue. Error: "))?;

    let resp = MovieDeletionRequest {
        radarr_response,
        overseerr_response,
        cache_response,
    };

    Ok(resp)
}
