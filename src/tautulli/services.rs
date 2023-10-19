use super::models::TautulliResponse;
use crate::deleterr::models::APIResponse;
use crate::deleterr::services::{make_api_call, map_to_api_response, process_request};
use actix_web::{get, web, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};
use std::time::Duration;

fn get_api_endpoint(endpoint: &str) -> Result<reqwest::RequestBuilder, Error> {
    dotenv().ok();

    let client = reqwest::Client::new();

    let tt_request_url = std::env::var("TT_REQUEST_URL").expect("os_request_url must be set.");
    let tt_api_key = std::env::var("TT_API_KEY").expect("os_api_key must be set.");

    let req_client = client
    .get(format!("{tt_request_url}{tt_api_key}{endpoint}"))
    .timeout(Duration::from_secs(15))
    .header(ACCEPT, "application/json");

    Ok(req_client)
}

pub async fn get_item_history() -> Result<APIResponse<TautulliResponse>, Error> {
    let endpoint = "&cmd=get_history&rating_key=29946";
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;
    let api_response = 
        map_to_api_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

#[get("/history")]
async fn get_requests_json() -> impl Responder {
    let requests_response = get_item_history().await;
    return process_request(requests_response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests_json);
}
