use std::fmt::Debug;

use super::models::{OverseerrResponse, Request};
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error, Response};

async fn make_api_call(endpoint: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(format!("{os_request_url}{endpoint}"))
        .header("X-Api-Key", "os_api_key")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .error_for_status()?;

    Ok(response)
}

async fn get_requests_response() -> Result<OverseerrResponse<Request>, Error> {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let response_result = make_api_call(&endpoint).await?;
    let response_text = response_result.json::<OverseerrResponse<Request>>().await?;
    Ok(response_text)
}

fn process_request<T>(requests: &Result<OverseerrResponse<T>, Error>) -> impl Responder
where
    T: serde::Serialize,
{
    return match requests {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            HttpResponse::InternalServerError().json(format!("{{error: '{}'}}", error.to_string()))
        }
    };
}

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let requests: Result<OverseerrResponse<Request>, Error> = get_requests_response().await;
    return process_request(&requests);
}

#[get("requests/all/text")]
async fn get_requests_text() -> impl Responder {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let response = make_api_call(&endpoint)
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
