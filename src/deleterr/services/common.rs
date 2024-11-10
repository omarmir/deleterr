use std::{collections::HashMap, sync::Arc};

use actix_web::web::Data;
use uuid::Uuid;

use crate::{
    common::{
        broadcast::{Broadcaster, MessageType, SSEType},
        models::deleterr_error::DeleterrError,
    },
    deleterr::{
        models::{AppData, CacheStatus, QueryParms, RequestStatusWithRecordInfo},
        requests::get_requests_and_update_cache,
    },
    overseerr::models::MediaType,
};

use super::{movie::get_request_status_for_movie, series::get_request_status_for_series};

pub async fn match_requests_to_watched(
    broadcaster: Arc<Broadcaster>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let (os_requests, page_info) = crate::overseerr::services::get_os_requests().await?;

    let total_len = os_requests.len();

    let mut matched_requests = HashMap::with_capacity(page_info.results);

    for i in 0..total_len {
        let media_request = &os_requests[i];
        let (media_type, tmdb_id, rating_key, user_id, tvdb_id) = (
            &media_request.media.media_type,
            &media_request.media.tmdb_id,
            &media_request.media.rating_key,
            &media_request.requested_by.plex_id,
            &media_request.media.tvdb_id,
        );

        let tau_hist =
            crate::tautulli::services::get_item_history(rating_key, user_id, media_type).await?;

        let request_status = match media_type {
            MediaType::TV => {
                // Get series from Sonarr
                let sonarr_series = crate::sonarr::services::get_series(tvdb_id).await?;
                // Get from Sonar series
                let series_id = match &sonarr_series {
                    Some(series) => Some(series.id),
                    None => None,
                };
                // Use the ID to get episodes from sonarr
                let episodes = crate::sonarr::services::get_episodes(&series_id).await;

                let eps = match episodes {
                    Ok(episodes) => episodes,
                    Err(err) => {
                        print!("{}", err);
                        None
                    }
                };

                let request_status =
                    get_request_status_for_series(media_request, sonarr_series, tau_hist, eps)?;

                request_status
            }
            MediaType::Movie => {
                let radarr_movie = crate::radarr::services::get_movie(tmdb_id).await?;
                let request_status =
                    get_request_status_for_movie(media_request, radarr_movie, tau_hist).await?;

                request_status
            }
        };

        matched_requests.insert(request_status.media_request.id, request_status);

        broadcaster
            .broadcast(MessageType::Progress((i + 1, total_len)))
            .await;
    }

    let request_status_with_record_info = RequestStatusWithRecordInfo {
        all_requests: page_info.results,
        requests: matched_requests,
    };

    Ok(request_status_with_record_info)
}

/// Broadcasts stream requests and handles completion or error messages. This will clone the completion once
/// the requests matching is done.
/// * This function checks first if the cache is built
/// * If built, it broadcasts the complete event and removes the client ending the SSE session
/// * If building, it doesn't do anything
/// * if uninitialized (not building/built) then it starts the build process
///     * When the build process is complete it removes all clients that may have subscribed to the requests event
///
/// # Parameters
/// * `app_data`: The app state of type [AppData]
/// * `client_id`: Unique identifier for the client.
pub async fn broadcast_stream_requests(
    app_data: Data<AppData>,
    client_id: Uuid,
    is_refresh: String,
) {
    let broadcaster = app_data.broadcaster.clone();
    let refresh_req = is_refresh == "refresh";
    let mut cache_status = app_data.cache_status();

    // If cache is build then you may request reset
    // If cache is uninit then just start building it
    // If cache is building then ignore refresh request
    match (refresh_req, cache_status) {
        (true, CacheStatus::Built) => {
            if let Err(err) = app_data.clear_cache() {
                broadcaster
                    .broadcast(MessageType::Error(err.to_string()))
                    .await;
                broadcaster.close_client(client_id);
                return;
            } else {
                cache_status = CacheStatus::Uninitialized;
            }
        }
        _ => (),
    }

    match cache_status {
        CacheStatus::Built => {
            broadcaster.broadcast(MessageType::Complete).await;
            broadcaster.close_client(client_id);
        }
        CacheStatus::Uninitialized => {
            println!("Building cache");
            app_data.set_cache_is_building();
            let resp = get_requests_and_update_cache(app_data.clone(), QueryParms::default()).await;

            match resp {
                Ok(_) => {
                    broadcaster.broadcast(MessageType::Complete).await;
                    app_data.set_cache_is_built();
                }
                Err(err) => {
                    broadcaster
                        .broadcast(MessageType::Error(err.to_string()))
                        .await;
                }
            };

            broadcaster.close_clients_of_type(SSEType::Requests);
        }
        CacheStatus::Building => {}
    }
}
