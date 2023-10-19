use actix_web::{get, web, Responder};
use crate::common::services::{map_to_api_response, process_request};
use crate::common::models::{APIResponse, APIData};
use crate::overseerr::models::{MediaRequest, OverseerrResponses, OverseerrListResponse};

use super::models::{Requests, RequestStatus};

async fn get_all_requests() {
    let os_requests = crate::os_serv::get_requests(1, 1).await.unwrap();
    let os_data: OverseerrListResponse<MediaRequest> = os_requests.data.into_success().unwrap().into_list().unwrap();
    let key = os_data.results[0].media.rating_key;
    let user_id = os_data.results[0].requested_by.plex_id;

    let tt = match (key, user_id) {
        (Some(rating_key), Some(user_id)) => Some(crate::tt_serv::get_item_history(rating_key, user_id).await.unwrap()),
        _ => None,
    };

    let tt_watched = tt.unwrap();


    let media_request = os_data.results[0].clone();
    let user_watch_history = tt_watched.data.as_success().unwrap().response.data.data.unwrap()[0].clone();


    let matched_result = RequestStatus {
        media_request,
        user_watch_history
    };
}

//#[get("/api/v1/json/requests/all")]
async fn get_all_requests_json()  { //-> impl Responder
}

pub fn config(cfg: &mut web::ServiceConfig) {
    //cfg.service(get_all_requests_json);
}