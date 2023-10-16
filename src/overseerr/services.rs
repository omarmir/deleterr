use super::models::{APIResponse, OverseerrResponse, Request};
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};

async fn make_api_call(endpoint: &str) -> Result<APIResponse, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(format!("{os_request_url}{endpoint}"))
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    let response_code = response.status().as_u16();

    let api_response = match response_code {
        200 => APIResponse {
            success: true,
            data: super::models::APIData::Success(
                response.json::<OverseerrResponse<Request>>().await?,
            ),
            code: response_code,
        },
        _ => APIResponse {
            success: false,
            data: super::models::APIData::Failure(response.text().await?),
            code: response_code,
        },
    };

    Ok(api_response)
}

async fn get_requests_response() -> Result<HttpResponse, Error> {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let response_result = make_api_call(&endpoint).await?;

    return Ok(HttpResponse::Ok().json(response_result));
}

fn process_request(requests: Result<HttpResponse, Error>) -> impl Responder {
    return match requests {
        Ok(response) => response,
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    };
}

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let requests = get_requests_response().await;
    return process_request(requests);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests);
}
