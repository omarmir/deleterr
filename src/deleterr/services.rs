use super::models::{
    AppData, CacheStatus, MediaInfo, MovieDeletionRequest, QueryParms, RequestStatus,
    RequestStatusWithRecordInfo, SeriesDeletionEpisodes, SeriesDeletionRequest,
};
use super::requests::{delete_cached_record, get_cached_record, get_requests_and_update_cache};
use super::watched::{SeasonWithStatus, WatchedChecker, WatchedStatus};
use crate::common::broadcast::{Broadcaster, MessageType, SSEType};
use crate::common::models::deleterr_error::DeleterrError;
use crate::overseerr::models::{MediaRequest, MediaType};
use crate::radarr::models::Movie;
use crate::sonarr::series::Series;
use crate::tautulli::user_watch_history::{
    ConvertToHashMapBySeason, GetFirstOrNone, UserWatchHistory,
};
use actix_web::web::Data;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_request_status_for_series(
    media_request: &MediaRequest,
    sonarr_series: Option<Series>,
    tau_hist: Option<Vec<UserWatchHistory>>,
) -> Result<RequestStatus, DeleterrError> {
    let (season_status, watched) = {
        let tau_history = tau_hist.hashmap_seasons();
        let mut seasons_with_status = vec![];
        let series_map = Series::hashmap_seasons(&sonarr_series);

        for season in &media_request.seasons {
            let series_season = series_map.get(&season.season_number);
            let watched_statuses = tau_history.get(&season.season_number);
            let season_with_status =
                SeasonWithStatus::from_series(season, watched_statuses, series_season);
            seasons_with_status.push(season_with_status)
        }

        let watched = seasons_with_status.is_watched(*&media_request.seasons.len());

        (seasons_with_status, watched)
    };

    let media_info = MediaInfo::from_sonarr(sonarr_series.clone());

    let mut request_status = RequestStatus {
        media_info,
        season_status,
        media_request: media_request.clone(),
        watched,
    };

    // We can make this a lot more efficient - fine for now
    // TODO: Doing this manually for now, but it should be done via setting later right now we are setting watched seasons by latest watched/In progress season

    let cloned_seasons = request_status.season_status.clone(); // Don't want to modify as I iterate, so a clone is made

    for season in &mut request_status.season_status {
        if let Some(season_number) = season.season_number {
            let next_season = cloned_seasons
                .iter()
                .find(|curr_season| curr_season.season_number == Some(season_number + 1));

            if let Some(sesn) = next_season {
                if sesn.watched == WatchedStatus::InProgress
                    || sesn.watched == WatchedStatus::Watched
                {
                    season.watched = WatchedStatus::Watched
                };
            }
        }
    }

    request_status.watched = request_status
        .season_status
        .is_watched(*&media_request.seasons.len());

    Ok(request_status)
}

pub async fn get_request_status_for_movie(
    media_request: &MediaRequest,
    radarr_movie: Option<Movie>,
    tau_hist: Option<Vec<UserWatchHistory>>,
) -> Result<RequestStatus, DeleterrError> {
    let watched = tau_hist.get_first_or_none().is_watched(1);
    let season_status = vec![SeasonWithStatus::from_movie(&watched, media_request)];
    let media_info = MediaInfo::from_radarr(radarr_movie);

    let request_status = RequestStatus {
        media_info,
        season_status,
        media_request: media_request.clone(),
        watched,
    };

    Ok(request_status)
}

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
                let sonarr_series = crate::sonarr::services::get_series(tvdb_id).await?;
                let request_status =
                    get_request_status_for_series(media_request, sonarr_series, tau_hist).await?;
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

pub async fn delete_movie_from_radarr_overseerr(
    app_data: &Data<AppData>,
    request_id: usize,
) -> Result<MovieDeletionRequest, DeleterrError> {
    // Get the record from the cache
    let request = get_cached_record(app_data, &request_id).ok_or(DeleterrError::new(
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
        .map_err(|err| err.add_prefix("Unble to delete from Radarr. Error: "))?;

    let overseerr_response = crate::overseerr::services::delete_media(media_id.to_string().as_str())
        .await
        .map_err(|err| err.add_prefix("Radarr media deleted but was unable to delete Overseer request. Please delete it manually. Error: "))?;

    let cache_response = delete_cached_record(app_data, &request_id)
        .map_err(|err| err.add_prefix("Deleted in both Radarr and Overseer but was unable to remove from Cache. Manually trigger a sync to fix the issue. Error: "))?;

    let resp = MovieDeletionRequest {
        radarr_response,
        overseerr_response,
        cache_response,
    };

    Ok(resp)
}

/// Looks at the current App State and calculates which episode files need deletion and if it the full request
/// that has been watched.
///
/// # Arguments:
///
/// * `request_id`: Used to identify a specific request for which we need to figure out watched seasons.
/// * `app_data`: The app state of type [AppData]
///
/// # Returns:
///
/// `Result` containing a list of episode file ids for deletion and a flag that indicates if all seasons are watched
/// which we can use to determine if we are OK to delete the entire request.
///
/// `Result` containing a [ResponseCodeBasedAction] or [DeleterrError] if there was an error during the process.
pub async fn get_watched_seasons_episodes(
    app_data: &Data<AppData>,
    request_id: &usize,
) -> Result<SeriesDeletionEpisodes, DeleterrError> {
    // Get the record from the cache
    let request = get_cached_record(app_data, request_id).ok_or(DeleterrError::new(
        "No request found! You may need to resync the services",
    ))?;

    // Make sure we have the series ID in the request cache
    let series_id = request
        .media_request
        .media
        .external_service_id
        .ok_or(DeleterrError::new(
            "No ID found in Sonarr. Manual deletion required.",
        ))?;

    // Get a vec of watched season numbers
    let watched_seasons: Vec<usize> = request
        .season_status
        .iter()
        .filter(|season| season.watched == WatchedStatus::Watched)
        .filter_map(|season| season.season_number)
        .collect();

    // We need to know if we are deleting the request all together or not - if all requested seasons watched then yes, otherwise no
    let is_request_fully_watched = watched_seasons.len() == request.media_request.seasons.len();

    // Get all episodes for a series
    let episode_files = crate::sonarr::services::get_episode_files(series_id)
        .await?
        .ok_or(DeleterrError::new(
            "No episode files found for this series in Sonarr.",
        ))?;

    // Get a vec of file ids for all episodes inside the watched season
    let watched_episodes_for_seasons: Vec<usize> = episode_files
        .iter()
        .filter(|file| watched_seasons.contains(&file.season_number))
        .map(|watched_eps| watched_eps.id)
        .collect();

    Ok(SeriesDeletionEpisodes {
        episodes: watched_episodes_for_seasons,
        request_fully_watched: is_request_fully_watched,
    })
}

pub async fn delete_watched_seasons_and_possibly_request(
    app_data: &Data<AppData>,
    request_id: &usize,
    episodes: SeriesDeletionEpisodes,
) -> Result<SeriesDeletionRequest, DeleterrError> {
    let sonarr_response = crate::sonarr::services::delete_episodes(episodes.episodes)
        .await
        .map_err(|err| err.add_prefix("Unble to delete from Sonarr. Error: "))?;

    let overseerr_response = if episodes.request_fully_watched {
        Some(crate::overseerr::services::delete_request(request_id.to_string().as_str())
        .await
        .map_err(|err| err.add_prefix("Sonarr media deleted but was unable to delete Overseer request. Please delete it manually. Error: "))?)
    } else {
        None
    };

    let cache_response = delete_cached_record(app_data, request_id)
        .map_err(|err| err.add_prefix("Deleted in both Radarr and Overseer but was unable to remove from Cache. Manually trigger a sync to fix the issue. Error: "))?;

    let resp = SeriesDeletionRequest {
        request_fully_watched: episodes.request_fully_watched,
        sonarr_response,
        cache_response,
        overseerr_response,
    };

    Ok(resp)
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
            broadcaster.broadcast(MessageType::Completion).await;
            broadcaster.close_client(client_id);
        }
        CacheStatus::Uninitialized => {
            println!("Building cache");
            app_data.set_cache_is_building();
            let resp = get_requests_and_update_cache(app_data.clone(), QueryParms::default()).await;

            match resp {
                Ok(_) => {
                    broadcaster.broadcast(MessageType::Completion).await;
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
