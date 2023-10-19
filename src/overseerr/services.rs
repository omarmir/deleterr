use std::time::Duration;

use super::models::{
    MediaRequest, OverseerrListResponse, OverseerrRequestsCount, OverseerrResponses,
};
use crate::deleterr::models::APIResponse;
use crate::deleterr::services::{make_api_call, process_request, map_to_api_response};
use actix_web::{get, web, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};

fn get_api_endpoint(endpoint: &str) -> Result<reqwest::RequestBuilder, Error> {
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

async fn get_requests() -> Result<APIResponse<OverseerrResponses<MediaRequest>>, Error> {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
        .json::<OverseerrListResponse<MediaRequest>>()
        .await?;
    let overseerr_response: OverseerrResponses<MediaRequest> = OverseerrResponses::List(resp);
    let api_response = map_to_api_response(
        overseerr_response,
        request_response.code,
        request_response.status,
    )
    .await?;
    Ok(api_response)
}

#[get("/requests/all")]
async fn get_requests_json() -> impl Responder {
    let requests_response = get_requests().await;
    return process_request(requests_response);
}

pub async fn get_requests_count() -> Result<APIResponse<OverseerrResponses<()>>, Error> {
    let endpoint = "request/count";
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
        .json::<OverseerrRequestsCount>()
        .await?;
    let overseerr_response: OverseerrResponses<()> = OverseerrResponses::Count(resp);
    let api_response = map_to_api_response(
        overseerr_response,
        request_response.code,
        request_response.status,
    )
    .await?;
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
