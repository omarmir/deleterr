use super::models::{AppData, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo};
use super::requests::{delete_cached_record, get_cached_record};
use super::watched::{WatchedEpisode, WatchedSeason};
use crate::common::models::DeleterrError;
use crate::overseerr::models::{MediaInfo, MediaRequest, MediaType};
use crate::sonarr::models::{Episode, SonarrShow};
use crate::tautulli::user_watch_history::{ConvertToHashMapBySeason, GetAllOrNone, GetFirstOrNone};
use actix_web::web::Data;
use std::collections::HashMap;

pub async fn get_request_status(
    rating_key: &Option<u64>,
    user_id: &Option<u64>,
    media_type: &MediaType,
    media_info: MediaInfo,
    media_request: &MediaRequest,
    all_episodes: Option<Vec<Episode>>,
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
        MediaType::TV => {
            let tau_history = tau_history_response
                .get_all_or_none()
                .convert_to_hash_map_by_season();
            let show = SonarrShow::from(all_episodes);
            let mut watched = vec![];
            for season in show.seasons.into_iter() {
                let mut watched_episodes: Vec<WatchedEpisode> = Vec::new();
                for episode in season.1.episodes {
                    let watched_episode = WatchedEpisode {
                        external_service_id: episode.1.id,
                        file_id: Some(episode.1.episode_file_id),
                        watched_status: tau_history.as_ref().map_or(0.0, |tau_hash| {
                            tau_hash
                                .get(&(season.0, episode.0))
                                .map_or(0.0, |hist| hist.watched_status)
                        }),
                        episode_number: Some(episode.0),
                        season_number: Some(season.0),
                    };
                    watched_episodes.push(watched_episode);
                }
                let watched_season = WatchedSeason {
                    season_number: Some(season.1.season_number),
                    req_status: media_request.status,
                    watched_episodes: Some(watched_episodes),
                    watched: false, // TODO: Change!
                    total_items: Some(season.1.episode_count),
                };

                watched.push(watched_season)
            }
            watched
        }
        MediaType::Movie => {
            let tau_history = tau_history_response.get_first_or_none();
            //let watched = tau_history.map_or(0.0, |hist| hist.watched_status);

            let watched_season = WatchedSeason {
                season_number: None,
                req_status: media_request.status,
                watched_episodes: Some(vec![WatchedEpisode {
                    external_service_id: media_request.media.external_service_id.unwrap(),
                    file_id: None,
                    watched_status: tau_history.map_or(0.0, |hist| hist.watched_status),
                    episode_number: None,
                    season_number: None,
                }]),
                watched: false, // TODO: Change!
                total_items: Some(1),
            };

            vec![watched_season]
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

        let all_episodes = match (media_type, media_request.media.external_service_id) {
            (MediaType::TV, Some(sonarr_id)) => {
                Some(crate::sonarr::services::get_episodes(sonarr_id.to_string().as_str()).await?)
            }
            _ => None,
        };

        let request_status = get_request_status(
            rating_key,
            user_id,
            media_type,
            media_info,
            media_request,
            all_episodes,
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
