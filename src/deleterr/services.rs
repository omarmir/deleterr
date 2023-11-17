use super::models::{AppData, MovieDeletionRequest, RequestStatus, RequestStatusWithRecordInfo};
use super::requests::{delete_cached_record, get_cached_record};
use crate::common::models::DeleterrError;
use crate::overseerr::models::{MediaRequest, PageInfo};
use crate::overseerr::services::delete_media;
use crate::rd_serv::delete_movie;
use crate::tautulli::models::UserWatchHistory;
use actix_web::web::Data;
use std::collections::HashMap;

async fn get_os_requests() -> Result<(Vec<MediaRequest>, PageInfo), DeleterrError> {
    let os_requests = crate::os_serv::get_requests().await?;
    let vec_requests = os_requests.results;
    let page_info = os_requests.page_info;
    Ok((vec_requests, page_info))
}

async fn get_tau_history_by_key_user(
    rating_key: &u64,
    user_id: &u64,
) -> Result<Option<UserWatchHistory>, DeleterrError> {
    let tt_match = crate::tt_serv::get_item_history(
        rating_key.to_string().as_str(),
        user_id.to_string().as_str(),
    )
    .await?;

    /*
     * We make sure that there is exactly one matched result since
     * we provided both a ratingKey and userId. If there is more than one result
     * then we did something wrong.
     */

    return match tt_match.response.data.data {
        Some(histories) => {
            if histories.len() == 1 {
                Ok(Some(histories[0].clone()))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    };
}

pub async fn match_requests_to_watched(
    chunk: Option<usize>,
) -> Result<RequestStatusWithRecordInfo, DeleterrError> {
    let chunk_size = chunk.unwrap_or(10);
    // ! Note that the default take is 10 at overseerr if unspecified!
    let (os_requests, page_info) = get_os_requests().await?;
    let mut matched_requests: HashMap<usize, RequestStatus> =
        HashMap::with_capacity(page_info.results);

    // TODO: Change this - maybe do a query first on the count?
    for i in 0..10 {
        //os_requests.len() {
        let media_request = &os_requests[i];
        let (media_type, tmdb_id, rating_key, user_id) = (
            &media_request.media.media_type,
            &media_request.media.tmdb_id,
            &media_request.media.rating_key,
            &media_request.requested_by.plex_id,
        );

        let media_info = crate::overseerr::services::get_media_info(&media_type, &tmdb_id).await?;
        let user_watch_history = match (rating_key, user_id) {
            (Some(rating_key), Some(user_id)) => {
                get_tau_history_by_key_user(&rating_key, &user_id).await?
            }
            _ => None,
        };

        let request_status = RequestStatus {
            media_info,
            user_watch_history,
            media_request: media_request.clone(),
        };

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
