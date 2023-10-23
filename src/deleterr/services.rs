use super::models::RequestStatus;
use crate::common::models::{APIData, APIResponse, DeleterrError};
use crate::common::services::{map_to_api_response, process_request};
use crate::deleterr::models::QueryParms;
use crate::overseerr::models::{MediaRequest, PageInfo};
use crate::tautulli::models::UserWatchHistory;
use actix_web::{get, web, Responder};

async fn get_os_requests(
    take: usize,
    skip: usize,
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

    let media_type = &media_request.media.media_type;
    let tmdb_id = &media_request.media.tmdb_id;

    let media_info = match (media_type, tmdb_id) {
        (Some(media_type), Some(tmdb_id)) => crate::os_serv::get_media_info(media_type, tmdb_id)
            .await
            .ok(),
        _ => None,
    };

    return match (rating_key, user_id) {
        (Some(rating_key), Some(user_id)) => RequestStatus {
            media_request: media_request.clone(),
            media_info,
            user_watch_history: get_tau_history_by_key_user(rating_key, user_id).await,
        },
        _ => RequestStatus {
            media_request: media_request.clone(),
            media_info,
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
async fn match_requests_to_watched(
    take: Option<usize>,
    skip: Option<usize>,
) -> Result<APIResponse<Vec<RequestStatus>>, DeleterrError> {
    //Our chunk size is 10 so that we don't DDOS our own server if we have thousands of requests
    // This app uses tokio's spawn function which is concurrent
    //We take the lesser of our specified take or 10
    //So that if we getting sent a take of 60,000 we do it 10 at a time
    let chunk_size = std::cmp::min(10, take.unwrap_or(10));
    let skip_val = skip.unwrap_or(0);

    let (os_requests, page_info) = get_os_requests(chunk_size, skip_val).await?;
    let result_or_take = std::cmp::min(page_info.results, take.unwrap_or(page_info.results)); //Length of the vector that holds the final result is the lesser or take or results
    let final_result_count = match result_or_take <= skip_val {
        false => result_or_take - skip_val,
        true => 0,
    };

    let mut matched_requests: Vec<RequestStatus> = Vec::with_capacity(final_result_count);

    if final_result_count == 0 {
        let api_response =
            map_to_api_response(matched_requests, 200, "Success".to_string()).await?;
        return Ok(api_response);
    }

    let max_pages = num_integer::div_ceil(final_result_count, chunk_size); // Calculates max pages by dividing the take by chunk size and rounding up
    let num_of_pages = std::cmp::min(page_info.pages, max_pages); // the lesser of the results or what we calculated on max_pages

    // Make the first match query to tautulli
    matched_requests.extend(make_tt_history_chunk_query(os_requests).await);

    // Less than/Equal to 10 requests requires no further requests to overseerr
    // so make an api response and sent it
    if final_result_count <= chunk_size {
        let api_response =
            map_to_api_response(matched_requests, 200, "Success".to_string()).await?;
        return Ok(api_response);
    }

    // If our max(take/results) > chunk size then we need to iterate
    // Start at 1 since we already did one query
    for i in 1..num_of_pages {
        // ! Sleep this for 2 seconds so we aren't just hammering the API endpoints
        tokio::time::sleep(std::time::Duration::from_secs(2)).await; // This doesn't spawn new threads - its green threading/tasking
        let skip = i * chunk_size;
        // Pick the lesser of what's left or the chunk size
        // e.g. if at 73 and skip 70 then pick 3 otherwise pick 10
        let take = std::cmp::min(final_result_count - skip, chunk_size);
        let (os_requests, _page_info) = get_os_requests(take, skip).await?;
        matched_requests.extend(make_tt_history_chunk_query(os_requests).await);
    }

    let api_response = map_to_api_response(matched_requests, 200, "Success".to_string()).await?;

    Ok(api_response)
}

#[get("/api/v1/json/requests")]
async fn get_all_requests_json(info: web::Query<QueryParms>) -> impl Responder {
    let matched_results = match_requests_to_watched(info.take, info.skip).await;
    return process_request(matched_results);
}

#[get("/api/v1/json/requests/count")]
async fn get_requests_count_json() -> impl Responder {
    let count_response = crate::os_serv::get_requests_count().await;
    return process_request(count_response);
}

#[get("/api/v1/json/requests/about/overseerr")]
async fn get_overseerr_about_json() -> impl Responder {
    let about_overseer = crate::os_serv::get_overseerr_about().await;
    return process_request(about_overseer);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json)
        .service(get_requests_count_json)
        .service(get_overseerr_about_json);
}
