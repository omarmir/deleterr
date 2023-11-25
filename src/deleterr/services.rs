use super::models::{AppData, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo};
use super::requests::{delete_cached_record, get_cached_record};
use crate::common::models::DeleterrError;
use crate::overseerr::models::{MediaInfo, MediaRequest, MediaType};
use crate::overseerr::seasons::AllSeasons;
use crate::tautulli::models::{GetAllOrNone, GetFirstOrNone};
use actix_web::web::Data;
use std::collections::HashMap;

pub async fn get_request_status(
    rating_key: &Option<u64>,
    user_id: &Option<u64>,
    media_type: &MediaType,
    media_info: MediaInfo,
    media_request: &MediaRequest,
    all_seasons: Option<AllSeasons>,
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

    let (movie_watch_history, episode_watch_history) = match media_type {
        MediaType::Movie => (tau_history_response.get_first_or_none(), None),
        MediaType::TV => (None, tau_history_response.get_all_or_none()),
    };

    let request_status = RequestStatus {
        media_info,
        movie_watch_history,
        episode_watch_history,
        media_request: media_request.clone(),
        all_seasons,
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

        /* ! This could result in unnessary multiple API calls
         * * Here we are pull the tv show info, in theory this could result in excess api calls since a show could have multiple requests
         * * but this is likley (?) an edge case so the additional overhead of creating and storing a hashmap and looking up there first
         * * and then falling back to the API seemed excessive. We shall see.
         */
        let all_seasons = match (media_type, media_request.media.tmdb_id) {
            (MediaType::TV, Some(tvid)) => {
                Some(crate::overseerr::services::get_seasons(tvid).await?)
            }
            _ => None,
        };

        let request_status = get_request_status(
            &rating_key,
            &user_id,
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
