use super::models::OverserrRequestResponse;
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error, Response};

async fn make_api_call() -> Result<Response, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(os_request_url)
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json")
        .send()
        .await;
    return response;
}

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let response = make_api_call()
        .await
        .expect("failed to get response")
        .json::<OverserrRequestResponse>()
        .await
        .expect("failed to get payload");
    HttpResponse::Ok().json(response)
}

#[get("requests/all/text")]
async fn get_requests_text() -> impl Responder {
    let response = make_api_call()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    HttpResponse::Ok().body(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests).service(get_requests_text);
}
