use std::time::Duration;

use super::models::{MediaRequest, OverseerrListResponse, OverseerrRequestsCount};
use crate::common::models::{APIResponse, DeleterrError};
use crate::common::services::{make_api_call, map_to_api_response, process_request};
use actix_web::{get, web, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};

fn get_api_endpoint(endpoint: String) -> Result<reqwest::RequestBuilder, Error> {
    dotenv().ok();

    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let req_client = reqwest::Client::new()
        .get(format!("{os_request_url}{endpoint}"))
        .timeout(Duration::from_secs(15))
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json");

    Ok(req_client)
}

pub async fn get_requests(
    take: u16,
    skip: u16,
) -> Result<APIResponse<OverseerrListResponse<MediaRequest>>, DeleterrError> {
    let endpoint = format!("request?take={take}&skip={skip}&sort=added&filter=available");
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
        .json::<OverseerrListResponse<MediaRequest>>()
        .await?;

    let api_response =
        map_to_api_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

#[get("/requests/{take}/{skip}")]
async fn get_requests_json(path: web::Path<(u16, u16)>) -> impl Responder {
    let (take, skip) = path.into_inner();
    let requests_response = get_requests(take, skip).await;
    return process_request(requests_response);
}

pub async fn get_requests_count() -> Result<APIResponse<OverseerrRequestsCount>, DeleterrError> {
    let endpoint = "request/count".to_string();
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
        .json::<OverseerrRequestsCount>()
        .await?;

    let api_response =
        map_to_api_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

#[get("/requests/count")]
async fn get_requests_count_json() -> impl Responder {
    let count_response = get_requests_count().await;
    return process_request(count_response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests_json)
        .service(get_requests_count_json);
}
