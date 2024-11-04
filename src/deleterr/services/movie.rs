use super::super::models::{AppData, MediaInfo, MovieDeletionRequest, RequestStatus};

use super::super::watched::{SeasonWithStatus, WatchedChecker};
use crate::common::models::deleterr_error::DeleterrError;
use crate::overseerr::models::MediaRequest;
use crate::radarr::models::Movie;
use crate::tautulli::user_watch_history::{GetFirstOrNone, UserWatchHistory};
use actix_web::web::Data;

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

pub async fn delete_movie_from_radarr_overseerr(
    app_data: &Data<AppData>,
    request_id: usize,
) -> Result<MovieDeletionRequest, DeleterrError> {
    // Get the record from the cache
    let request = app_data.get_request(&request_id).ok_or(DeleterrError::new(
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

    let cache_response = app_data.delete_request( &request_id)
        .map_err(|err| err.add_prefix("Deleted in both Radarr and Overseer but was unable to remove from Cache. Manually trigger a sync to fix the issue. Error: "))?;

    let resp = MovieDeletionRequest {
        radarr_response,
        overseerr_response,
        cache_response,
    };

    Ok(resp)
}
