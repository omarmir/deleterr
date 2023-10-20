use crate::common::models::{APIData, APIResponse};
use crate::common::services::{map_to_api_response, process_request};
use crate::overseerr::models::{MediaRequest, OverseerrListResponse, OverseerrResponses};
use actix_web::{get, web, Responder};

use super::models::RequestStatus;

async fn get_all_requests() -> Result<APIResponse<Vec<RequestStatus>>, reqwest::Error> {
    let os_requests = crate::os_serv::get_requests(1, 1).await.unwrap();
    let os_data: OverseerrListResponse<MediaRequest> = os_requests
        .data
        .into_success()
        .unwrap()
        .into_list()
        .unwrap();
    let rating_key = os_data.results[0].media.rating_key.unwrap();
    let user_id = os_data.results[0].requested_by.plex_id.unwrap();
    let tt_request = crate::tt_serv::get_item_history(rating_key, user_id)
        .await
        .unwrap();
    let tt_data = tt_request
        .data
        .into_success()
        .unwrap()
        .response
        .data
        .data
        .unwrap();

    let media_request = os_data.results[0].clone();
    let user_watch_history = tt_data[0].clone();

    let matched_result = RequestStatus {
        media_request,
        user_watch_history,
    };

    let requests = vec![matched_result];

    let api_response = APIResponse {
        success: true,
        code: 200,
        data: APIData::Success(requests),
    };

    Ok(api_response)
}

#[get("/api/v1/json/requests/all")]
async fn get_all_requests_json() -> impl Responder {
    let count_response = get_all_requests().await;
    return process_request(count_response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json);
}
