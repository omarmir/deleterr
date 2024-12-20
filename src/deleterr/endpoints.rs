use crate::auth::models::User;
use crate::common::broadcast::SSEType;
use crate::common::models::services::{ServiceInfo, Services};
use crate::common::services::process_request;
use crate::deleterr::models::QueryParms;
use crate::deleterr::services::{
    broadcast_stream_requests, delete_watched_seasons_and_possibly_request,
};
use crate::store::models::settings::Settings;
use crate::{auth, deleterr, overseerr, radarr, sonarr, sonrad, store, tautulli, AppData};
use actix_session::Session;
use actix_web::middleware::from_fn;
use actix_web::{
    delete, get, post,
    web::{self, Data},
    Responder,
};
use uuid::Uuid;

#[get("/requests")]
async fn get_all_requests_json(
    app_data: Data<AppData>,
    info: web::Query<QueryParms>,
) -> impl Responder {
    let matched_results =
        deleterr::requests::get_requests_and_update_cache(app_data, info.into_inner()).await;
    return process_request(matched_results);
}

/// Registers the client for SSE providing status updates. This bit is a bit complex:
/// * We first give the client a UUID and store this UUID along with the type of request this is
/// * We then add this client to the broadcaster inside the AppData so it gets updates
/// * We then spawn the `broadcast_stream_requests` function
/// ! There may be a race condition here but I haven't seen it play out where we may transmit messages
/// ! before the return has occured and thus the client may miss the updates
///
/// # Arguments:
///
/// * `app_data`: The `app_data` parameter is of type [Data]<[AppData]>, which contains shared
/// application data - the cache for the synced APIs. You don't need to provide this, Actix will handle this.
///
/// # Returns
/// Added client so the browser knows its an SSE
///
#[get("/requests/sse/{refresh}")]
async fn get_register_requests(
    app_data: Data<AppData>,
    refresh: web::Path<String>,
) -> impl Responder {
    let broadcaster = app_data.broadcaster.clone();
    let id = Uuid::new_v4();
    let add_client = broadcaster.new_client(id, SSEType::Requests).await;

    let _handle = actix_rt::spawn(broadcast_stream_requests(
        app_data,
        id,
        refresh.into_inner(),
    ));

    return add_client;
}

#[get("/requests/count")]
async fn get_requests_count_json() -> impl Responder {
    let count_response = overseerr::services::get_requests_count().await;
    return process_request(count_response);
}

#[post("/service/status")]
async fn get_service_status_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let service_status = match service_info.service {
        Services::Overseerr => overseerr::services::get_overseerr_status(service_info).await,
        Services::Tautulli => tautulli::services::get_tautulli_status(service_info).await,
        Services::Radarr => sonrad::services::get_sonrad_status(service_info).await,
        Services::Sonarr => sonrad::services::get_sonrad_status(service_info).await,
    };
    return process_request(service_status);
}

#[post("/service/save")]
async fn save_service_submit_json(
    web::Json(service_info): web::Json<ServiceInfo>,
) -> impl Responder {
    let inserted_result = store::services::services::upsert_service(service_info);
    return process_request(inserted_result);
}

#[get("/service/get")]
async fn get_all_service_json() -> impl Responder {
    let service_info = store::services::services::get_all_services();
    return process_request(service_info);
}

#[get("/service/get/{service_name}")]
async fn get_service_json(path: web::Path<Services>) -> impl Responder {
    let service_info = store::services::services::get_service(path.into_inner());
    return process_request(service_info);
}

#[get("/request/exemptions/get")]
async fn get_media_exemption() -> impl Responder {
    let media_exemptions = store::services::media_exemptions::get_all_exemptions();
    return process_request(media_exemptions);
}

#[post("/request/exemptions/save")]
async fn save_media_exemption(web::Json(media_exemption): web::Json<usize>) -> impl Responder {
    let exempted_result = store::services::media_exemptions::add_media_exemption(media_exemption);
    return process_request(exempted_result);
}

#[post("/request/exemptions/remove")]
async fn remove_media_exemption(web::Json(request_id): web::Json<usize>) -> impl Responder {
    let deleted_result = store::services::media_exemptions::remove_media_exemption(request_id);
    return process_request(deleted_result);
}

#[delete("/movie/delete/{media_id}")]
async fn delete_movie_file(app_data: Data<AppData>, path: web::Path<usize>) -> impl Responder {
    let delete_movie =
        deleterr::services::delete_movie_from_radarr_overseerr(&app_data, path.into_inner()).await;
    return process_request(delete_movie);
}

// Check out actix_proxy: https://docs.rs/actix-proxy/latest/actix_proxy/
// Tested - no difference in response times.
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

#[get("/movie/poster/{movie_id}/poster.jpg")]
async fn get_movie_poster(path: web::Path<usize>) -> actix_web::HttpResponse {
    let img = radarr::services::get_cover(path.into_inner())
        .await
        .unwrap();

    // Set the Content-Type header to image/png (adjust if your image format is different)
    actix_web::HttpResponse::Ok()
        .content_type("image/png")
        .body(img)
}

/// Deletes watched seasons of a series based on the provided series ID.
///
/// # Arguments:
///
/// * `app_data`: The `app_data` parameter is of type [Data]<[AppData]>, which contains shared
/// application data - the cache for the synced APIs. You don't need to provide this, Actix will handle this.
/// * `path`: The `path` parameter in the function is the `series_id` of the TV series for which the watched seasons are being deleted.
/// It is extracted from the URL path as a `usize` type.
///
/// <div class="warning">Please use caution this can delete multiple files and may take a while!</div>
///
#[delete("/series/{series_id}/delete/seasons/watched")]
async fn delete_watched_seasons(app_data: Data<AppData>, path: web::Path<usize>) -> impl Responder {
    let req_id = path.into_inner();
    // Calculate the watched seasons and if the whole request is watched
    let episodes_for_deletion =
        deleterr::services::get_watched_seasons_episodes(&app_data, &req_id).await;

    // Check if successful
    let processed_request = match episodes_for_deletion {
        Ok(episodes) => {
            // Delete the watched seasons and POSSIBLY the request if all watched in request.
            let deleted_episodes =
                delete_watched_seasons_and_possibly_request(&app_data, &req_id, episodes).await;
            process_request(deleted_episodes)
        }
        Err(err) => process_request(Err(err)),
    };

    return processed_request;
}

// Settings
#[post("/settings/save")]
async fn save_settings_submit_json(web::Json(settings): web::Json<Settings>) -> impl Responder {
    let saved_result = store::services::settings::save_settings(settings);
    return process_request(saved_result);
}

#[get("/settings/get")]
async fn get_all_settings_json() -> impl Responder {
    let settings = store::services::settings::get_all_settings();
    return process_request(settings);
}

// Overseerr services
#[get("/overseerr/get/radarr")]
async fn get_overseerr_radar_info() -> impl Responder {
    let radarr_info = overseerr::services::get_overseerr_radar_info().await;
    return process_request(radarr_info);
}

// Auth
#[post("/auth/login")]
async fn set_login(session: Session, web::Json(user): web::Json<User>) -> impl Responder {
    let is_login_success = auth::services::login_user(session, user);

    return process_request(is_login_success);
}

#[post("/auth/logout")]
async fn set_logout(session: Session) -> impl Responder {
    let resp = auth::services::logout_user(session);
    return process_request(resp);
}

#[post("/auth/user/add")]
async fn add_user(web::Json(user): web::Json<User>) -> impl Responder {
    let resp = store::services::users::add_user(user);
    return process_request(resp);
}

#[post("/auth/user/password")]
async fn update_password(
    session: Session,
    web::Json(new_password): web::Json<String>,
) -> impl Responder {
    let is_login_success = auth::services::update_password(session, new_password);

    return process_request(is_login_success);
}

#[post("/auth/user/validate")]
async fn validate_user_session(
    session: Session,
    web::Json(username): web::Json<String>,
) -> impl Responder {
    let is_login_success = auth::services::validate_session(session, username);

    return process_request(is_login_success);
}

#[get("/auth/setup/status")]
async fn get_setup_status() -> impl Responder {
    let is_users_setup = store::services::users::is_users_setup();
    return process_request(is_users_setup);
}

#[post("/auth/user/initialize")]
async fn initialize_user(web::Json(user): web::Json<User>) -> impl Responder {
    let resp = store::services::users::initialize_user(user);
    return process_request(resp);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/json")
            .wrap(from_fn(auth::services::reject_anonymous_users))
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
            .service(add_user)
            .service(validate_user_session)
            .service(get_series_poster)
            .service(get_movie_poster)
            .service(save_settings_submit_json)
            .service(get_all_settings_json)
            .service(get_register_requests)
            .service(update_password)
            .service(get_overseerr_radar_info)
            .service(delete_watched_seasons),
    )
    .service(set_login)
    .service(set_logout)
    .service(get_setup_status)
    .service(initialize_user);
}
