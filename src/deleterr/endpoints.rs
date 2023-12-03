use crate::auth::models::User;
use crate::auth::services::{
    login_user, logout_user, reject_anonymous_users, upsert_user, validate_session,
};
use crate::common::models::MediaExemption;
use crate::common::{models::ServiceInfo, models::Services, services::process_request};
use crate::deleterr::models::QueryParms;
use crate::deleterr::requests::get_requests_and_update_cache;
use crate::{sonarr, AppData};
use actix_session::Session;
use actix_web::{
    delete, get, post,
    web::{self, Data},
    Responder,
};
use actix_web_lab::middleware::from_fn;

#[get("/requests")]
async fn get_all_requests_json(
    app_data: Data<AppData>,
    info: web::Query<QueryParms>,
) -> impl Responder {
    let matched_results = get_requests_and_update_cache(app_data, info.into_inner()).await;
    return process_request(matched_results);
}

#[get("/requests/count")]
async fn get_requests_count_json() -> impl Responder {
    let count_response = crate::overseerr::services::get_requests_count().await;
    return process_request(count_response);
}

#[post("/service/status")]
async fn get_service_status_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let service_status = match service_info.service {
        Services::Overseerr => crate::overseerr::services::get_overseerr_status(service_info).await,
        Services::Tautulli => crate::tautulli::services::get_tautulli_status(service_info).await,
        Services::Radarr => crate::sonrad::services::get_sonrad_status(service_info).await,
        Services::Sonarr => crate::sonrad::services::get_sonrad_status(service_info).await,
    };
    return process_request(service_status);
}

#[post("/service/save")]
async fn save_service_submit_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let inserted_result = crate::store::services::upsert_service(service_info);
    return process_request(inserted_result);
}

#[get("/service/get")]
async fn get_all_service_json() -> impl Responder {
    let service_info = crate::store::services::get_all_services();
    return process_request(service_info);
}

#[get("/service/get/{service_name}")]
async fn get_service_json(path: web::Path<Services>) -> impl Responder {
    let service_info = crate::store::services::get_service(path.into_inner());
    return process_request(service_info);
}

#[get("/request/exemptions/get")]
async fn get_media_exemption() -> impl Responder {
    let media_exemptions = crate::store::exemptions::get_all_exemptions();
    return process_request(media_exemptions);
}

#[post("/request/exemptions/save")]
async fn save_media_exemption(
    web::Json(media_exemption): web::Json<MediaExemption>,
) -> impl Responder {
    let exempted_result = crate::store::exemptions::upsert_media_exemption(media_exemption);
    return process_request(exempted_result);
}

#[post("/request/exemptions/remove")]
async fn remove_media_exemption(web::Json(request_id): web::Json<usize>) -> impl Responder {
    let deleted_result = crate::store::exemptions::remove_media_exemption(request_id);
    return process_request(deleted_result);
}

#[delete("/movie/delete/{media_id}")]
async fn delete_movie_file(app_data: Data<AppData>, path: web::Path<usize>) -> impl Responder {
    let delete_movie =
        crate::deleterr::services::delete_movie_from_radarr_overseerr(&app_data, path.into_inner())
            .await;
    return process_request(delete_movie);
}

// TODO: check out actix_proxy: https://docs.rs/actix-proxy/latest/actix_proxy/
#[get("/series/poster/{series_id}/poster.jpg")]
async fn get_series_poster(path: web::Path<usize>) -> actix_web::HttpResponse {
    let img = sonarr::services::get_cover(path.into_inner())
        .await
        .unwrap();

    // Set the Content-Type header to image/png (adjust if your image format is different)
    actix_web::HttpResponse::Ok()
        .content_type("image/png")
        .body(img)
}

// Auth
#[post("/auth/login")]
async fn set_login(session: Session, web::Json(user): web::Json<User>) -> impl Responder {
    let is_login_success = login_user(session, user);

    return process_request(is_login_success);
}

#[post("/auth/logout")]
async fn set_logout(session: Session) -> impl Responder {
    let resp = logout_user(session);
    return process_request(resp);
}

#[post("/auth/user/save")]
async fn save_user(web::Json(user): web::Json<User>) -> impl Responder {
    let resp = upsert_user(user);
    return process_request(resp);
}

#[post("/auth/user/validate")]
async fn validate_user_session(
    session: Session,
    web::Json(username): web::Json<String>,
) -> impl Responder {
    let is_login_success = validate_session(session, username);

    return process_request(is_login_success);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/json")
            .wrap(from_fn(reject_anonymous_users))
            .service(get_all_requests_json)
            .service(get_requests_count_json)
            .service(get_service_status_json)
            .service(save_service_submit_json)
            .service(get_service_json)
            .service(get_all_service_json)
            .service(get_media_exemption)
            .service(save_media_exemption)
            .service(remove_media_exemption)
            .service(delete_movie_file)
            .service(save_user)
            .service(validate_user_session)
            .service(get_series_poster),
    )
    .service(set_login)
    .service(set_logout);
}
