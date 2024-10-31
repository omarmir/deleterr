use super::models::{
    AppData, MediaInfo, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo,
};
use super::requests::{delete_cached_record, get_cached_record};
use super::watched::{SeasonWithStatus, WatchedChecker, WatchedStatus};
use crate::common::models::deleterr_error::DeleterrError;
use crate::overseerr::models::{MediaRequest, MediaType};
use crate::radarr::models::Movie;
use crate::sonarr::series::Series;
use crate::sonarr::services::get_episode_files;
use crate::tautulli::user_watch_history::{
    ConvertToHashMapBySeason, GetFirstOrNone, UserWatchHistory,
};
use actix_web::web::Data;
use std::collections::HashMap;

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

    let request_status = RequestStatus {
        media_info,
        season_status,
        media_request: media_request.clone(),
        watched,
    };

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

pub async fn match_requests_to_watched() -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let (os_requests, page_info) = crate::overseerr::services::get_os_requests().await?;
    let mut matched_requests = HashMap::with_capacity(page_info.results);

    for i in 0..os_requests.len() {
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
        .map_err(|err| err.add_prefix("Unble to delete from Radarr. Error: "))?;

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

/// Looks at the current App State and deletes any seasons where episodes are watched
///
/// <div class="warning">THIS IS DESTRUCTIVE. If its a lot of episodes, it may take a long time. CAUTION!</div>
///
/// # Arguments:
///
/// * `request_id`: Used to identify a specific request for which we need to figure out watched seasons.
/// * `app_data`: The app state of type [AppData]
///
/// # Returns:
///
/// `Result` containing a [ResponseCodeBasedAction] or [DeleterrError] if there was an error during the process.
pub async fn delete_watched_seasons(
    app_data: &Data<AppData>,
    request_id: usize,
) -> Result<Vec<usize>, DeleterrError> {
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

    // Get all episodes for a series
    let episode_files = get_episode_files(series_id)
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

    Ok(watched_episodes_for_seasons)
}
