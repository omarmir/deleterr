use super::services::match_requests_to_watched;
use crate::common::services::process_request;
use crate::deleterr::models::QueryParms;
use actix_web::{get, post, web, Responder};

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

#[post("/api/v1/json/requests/status/overseerr")]
async fn get_overseerr_about_json() -> impl Responder {
    let about_overseer = crate::os_serv::get_overseerr_status().await;
    return process_request(about_overseer);
}

#[post("/api/v1/json/requests/status/tautulli")]
async fn get_status_tautulli_json() -> impl Responder {
    let status_tautulli = crate::tt_serv::get_tautulli_status().await;
    return process_request(status_tautulli);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json)
        .service(get_requests_count_json)
        .service(get_overseerr_about_json)
        .service(get_status_tautulli_json);
}
