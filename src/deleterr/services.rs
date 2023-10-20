use super::models::RequestStatus;
use crate::common::models::{APIData, APIResponse, DeleterrError};
use crate::common::services::{map_to_api_response, process_request};
use crate::overseerr::models::{MediaRequest, PageInfo};
use crate::tautulli::models::UserWatchHistory;
use actix_web::{get, web, Responder};

async fn get_os_requests(
    take: u16,
    skip: u16,
) -> Result<(Vec<MediaRequest>, PageInfo), DeleterrError> {
    let os_requests = crate::os_serv::get_requests(take, skip).await?.data;
    match os_requests {
        APIData::Success(data) => {
            let requests = data.results;
            let page_info = data.page_info;
            Ok((requests, page_info))
        }
        APIData::Failure(err) => Err(DeleterrError::new(err.as_str())),
    }
}

async fn get_tau_history_by_key_user(rating_key: u64, user_id: u64) -> Option<UserWatchHistory> {
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

async fn get_request_status_for_media_request(media_request: MediaRequest) -> RequestStatus {
    let rating_key = media_request.media.rating_key;
    let user_id = media_request.requested_by.plex_id;

    return match (rating_key, user_id) {
        (Some(rating_key), Some(user_id)) => RequestStatus {
            media_request: media_request.clone(),
            user_watch_history: get_tau_history_by_key_user(rating_key, user_id).await,
        },
        _ => RequestStatus {
            media_request: media_request.clone(),
            user_watch_history: None,
        },
    };
}

///Returns the requests matched with all the relevant watched status where available
///
/// # Arguments
///
///  * `os_requests` - A Vec of all the MediaRequests
async fn make_tt_history_chunk_query(os_requests: Vec<MediaRequest>) -> Vec<RequestStatus> {
    let os_requests_len = os_requests.len();
    let mut requests: Vec<RequestStatus> = Vec::with_capacity(os_requests_len);

    let mut handles = Vec::with_capacity(os_requests.len());

    for i in 0..os_requests.len() {
        let media_request: MediaRequest = os_requests[i].clone();
        let matched_request = get_request_status_for_media_request(media_request);
        handles.push(tokio::spawn(matched_request));
    }

    for handle in handles {
        let result = handle.await;
        match result {
            Ok(req_status) => requests.push(req_status),
            Err(_) => (),
        }
    }

    return requests;
}

/*
 * This function splits up the OS requests in 10 request chunks and then
 * spawns 10 threads (in theory)
 */
async fn match_requests_to_watched() -> Result<APIResponse<Vec<RequestStatus>>, DeleterrError> {
    let take: u16 = 10;
    let skip: u16 = 0;
    let (os_requests, page_info) = get_os_requests(take, skip).await?;
    let req_status_len = page_info.results;
    let num_of_pages = page_info.pages;
    let mut matched_requests: Vec<RequestStatus> = Vec::with_capacity(req_status_len);

    /*
     * Less than/Equal to 10 requests requires no further requests to overseerr
     * so make an api response and sent it
     */
    matched_requests.extend(make_tt_history_chunk_query(os_requests).await);

    if req_status_len < 11 {
        let api_response =
            map_to_api_response(matched_requests, 200, "Success".to_string()).await?;
        return Ok(api_response);
    }

    for i in 1..num_of_pages {
        // ! Sleep this for 2 seconds so we aren't just hammering the API endpoints
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let (os_requests, _page_info) = get_os_requests(take, i * skip).await?;
        matched_requests.extend(make_tt_history_chunk_query(os_requests).await);
    }

    let api_response = map_to_api_response(matched_requests, 200, "Success".to_string()).await?;

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
