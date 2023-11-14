use crate::common::models::MediaExemption;
use crate::common::services::send_response;
use crate::common::{models::ServiceInfo, models::Services, services::process_request};
use crate::deleterr::models::QueryParms;
use crate::deleterr::requests::get_requests_and_update_cache;
use crate::AppData;
use actix_web::{
    delete, get, post,
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
    // TODO: We haven't handled Tautulli with proper info yet, and have not done Sonarr or Radarr yet
    let service_status = match service_info.service {
        Services::Overseerr => crate::os_serv::get_overseerr_status(service_info).await,
        Services::Tautulli => crate::tt_serv::get_tautulli_status().await,
        _ => crate::os_serv::get_overseerr_status(service_info).await,
    };
    return process_request(service_status);
}

#[post("/api/v1/json/service/save")]
async fn save_service_submit_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let inserted_result = crate::st_serv::upsert_service(service_info);
    return process_request(inserted_result);
}

#[get("/api/v1/json/service/get")]
async fn get_all_service_json() -> impl Responder {
    let service_info = crate::st_serv::get_all_services();
    return process_request(service_info);
}

#[get("/api/v1/json/service/get/{service_name}")]
async fn get_service_json(path: web::Path<Services>) -> impl Responder {
    let service_info = crate::st_serv::get_service(path.into_inner());
    return process_request(service_info);
}

#[get("/api/v1/json/request/exemptions/get")]
async fn get_media_exemption() -> impl Responder {
    let media_exemptions = crate::st_exempt::get_all_exemptions();
    return process_request(media_exemptions);
}

#[post("/api/v1/json/request/exemptions/save")]
async fn save_media_exemption(
    web::Json(media_exemption): web::Json<MediaExemption>,
) -> impl Responder {
    let exempted_result = crate::st_exempt::upsert_media_exemption(media_exemption);
    return process_request(exempted_result);
}

#[post("/api/v1/json/request/exemptions/remove")]
async fn remove_media_exemption(web::Json(request_id): web::Json<usize>) -> impl Responder {
    let deleted_result = crate::st_exempt::remove_media_exemption(request_id);
    return process_request(deleted_result);
}

#[delete("/api/v1/json/movie/delete/{media_id}")]
async fn delete_movie_file(app_data: Data<AppData>, path: web::Path<usize>) -> impl Responder {
    let delete_movie =
        crate::dr_serv::delete_movie_from_radarr_overseerr(&app_data, path.into_inner()).await;
    return process_request(delete_movie);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_requests_json)
        .service(get_requests_count_json)
        .service(get_service_status_json)
        .service(save_service_submit_json)
        .service(get_service_json)
        .service(get_all_service_json)
        .service(get_media_exemption)
        .service(save_media_exemption)
        .service(remove_media_exemption)
        .service(delete_movie_file);
}
