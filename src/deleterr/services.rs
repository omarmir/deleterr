use super::models::{AppData, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo};
use super::requests::{delete_cached_record, get_cached_record};
use crate::common::models::DeleterrError;
use crate::overseerr::models::{MediaInfo, MediaRequest, MediaType, PageInfo};
use crate::overseerr::services::delete_media;
use crate::rd_serv::delete_movie;
use crate::tautulli::models::{GetAllOrNone, GetFirstOrNone};
use crate::tautulli::services::get_item_history;
use actix_web::web::Data;
use std::collections::HashMap;

async fn get_os_requests() -> Result<(Vec<MediaRequest>, PageInfo), DeleterrError> {
    let os_requests = crate::os_serv::get_requests().await?;
    let vec_requests = os_requests.results;
    let page_info = os_requests.page_info;
    Ok((vec_requests, page_info))
}

pub async fn get_request_status(
    rating_key: &Option<u64>,
    user_id: &Option<u64>,
    media_type: &MediaType,
    media_info: MediaInfo,
    media_request: &MediaRequest,
) -> Result<RequestStatus, DeleterrError> {
    let tau_history_response = match (rating_key, user_id) {
        (Some(rk), Some(uid)) => Some(
            get_item_history(
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
    };

    Ok(request_status)
}

pub async fn match_requests_to_watched(
    chunk: Option<usize>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let chunk_size = chunk.unwrap_or(10);
    // ! Note that the default take is 10 at overseerr if unspecified!
    let (os_requests, page_info) = get_os_requests().await?;
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

        let request_status =
            get_request_status(&rating_key, &user_id, media_type, media_info, media_request)
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

    let radarr_response = delete_movie(radarr_id.to_string().as_str())
        .await
        .map_err(|err| err.add_prefix("Unable to delete from Radarr. Error: "))?;

    let overseerr_response = delete_media(media_id.to_string().as_str())
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
