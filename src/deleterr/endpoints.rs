use crate::common::{models::ServiceInfo, models::Services, services::process_request};
use crate::deleterr::models::QueryParms;
use crate::deleterr::requests::get_requests_and_update_cache;
use crate::AppData;
use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};

#[get("/api/v1/json/requests")]
async fn get_all_requests_json(
    app_data: Data<AppData>,
    info: web::Query<QueryParms>,
) -> impl Responder {
    let matched_results = get_requests_and_update_cache(app_data, info.into_inner()).await;
    return process_request(matched_results);
}

#[get("/api/v1/json/requests/count")]
async fn get_requests_count_json() -> impl Responder {
    let count_response = crate::os_serv::get_requests_count().await;
    return process_request(count_response);
}

#[post("/api/v1/json/service/status")]
async fn get_service_status_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let service_status = match service_info.service {
        Services::Overseerr => crate::os_serv::get_overseerr_status(service_info).await,
        Services::Tautulli => crate::tt_serv::get_tautulli_status().await,
    };
    return process_request(service_status);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json)
        .service(get_requests_count_json)
        .service(get_service_status_json);
}
