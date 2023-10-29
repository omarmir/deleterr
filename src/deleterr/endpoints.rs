use super::models::RequestStatusWithRecordInfo;
use super::services::match_requests_to_watched;
use crate::common::models::{APIResponse, DeleterrError};
use crate::common::services::map_to_api_response;
use crate::common::{models::ServiceInfo, models::Services, services::process_request};
use crate::deleterr::models::QueryParms;
use crate::AppData;
use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use std::mem::drop;

async fn check_cache(
    app_data: Data<AppData>,
    take: Option<usize>,
) -> Result<APIResponse<RequestStatusWithRecordInfo>, DeleterrError> {
    let cache = app_data
        .request_cache
        .read()
        .expect("Unable to access cache");

    let unlocked_cache = &*cache;

    match unlocked_cache {
        Some(requests) => {
            let cloned = requests.clone();
            println!("Sending cache");
            let api_response = APIResponse {
                success: true,
                data: crate::common::models::APIData::Success(cloned),
                code: 200,
            };
            return Ok(api_response);
        }
        None => {
            let resp = match_requests_to_watched(take).await.unwrap();
            drop(cache);
            let mut update_cache = app_data
                .request_cache
                .write()
                .expect("Unable to access cache");

            *update_cache = Some(resp.clone());
            drop(update_cache);

            let api_response = map_to_api_response(resp, 200, "Success".to_string()).await?;

            return Ok(api_response);
        }
    }
}

#[get("/api/v1/json/requests")]
async fn get_all_requests_json(
    app_data: Data<AppData>,
    info: web::Query<QueryParms>,
) -> impl Responder {
    let matched_results = check_cache(app_data, info.take).await;
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
