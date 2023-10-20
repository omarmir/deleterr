use crate::common::models::{APIData, APIResponse, DeleterrError};
use crate::common::services::{map_to_api_response, process_request};
use crate::overseerr::models::MediaRequest;
use crate::tautulli::models::UserWatchHistory;
use actix_web::{get, web, Responder};

use super::models::RequestStatus;

async fn get_os_requests(take: u16, skip: u16) -> Result<Vec<MediaRequest>, DeleterrError> {
    let os_requests = crate::os_serv::get_requests(take, skip).await?.data;
    match os_requests {
        APIData::Success(data) => Ok(data.results),
        APIData::Failure(err) => Err(DeleterrError::new(err.as_str())),
    }
}

async fn get_tt_match(rating_key: u64, user_id: u64) -> Option<UserWatchHistory> {
    let tt_request = crate::tt_serv::get_item_history(rating_key, user_id)
        .await
        .ok()?
        .data;

    let tt_matches = match tt_request {
        APIData::Success(tt_response) => tt_response.response.data.data,
        _ => None,
    };

    /*
     * We make sure that there is exactly one matched result since
     * we provided both a ratingKey and userId. If there is more than one result
     * then we did something wrong.
     */
    return match tt_matches {
        Some(histories) => {
            if histories.len() == 1 {
                Some(histories[0].clone())
            } else {
                None
            }
        }
        _ => None,
    };
}

async fn match_os_requests(media_request: &MediaRequest) -> Option<UserWatchHistory> {
    let rating_key = media_request.media.rating_key;
    let user_id = media_request.requested_by.plex_id;

    return match (rating_key, user_id) {
        (Some(rating_key), Some(user_id)) => get_tt_match(rating_key, user_id).await,
        _ => None,
    };
}

async fn match_requests_to_watched() -> Result<APIResponse<Vec<RequestStatus>>, DeleterrError> {
    let os_requests = get_os_requests(1, 1).await?;

    let media_request = os_requests[0].clone();
    let user_watch_history = match_os_requests(&media_request).await;

    let matched_result = RequestStatus {
        media_request,
        user_watch_history,
    };

    let requests = vec![matched_result];

    let api_response = map_to_api_response(requests, 200, "Success".to_string()).await?;

    Ok(api_response)
}

#[get("/api/v1/json/requests/all")]
async fn get_all_requests_json() -> impl Responder {
    let matched_results = match_requests_to_watched().await;
    return process_request(matched_results);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json);
}
