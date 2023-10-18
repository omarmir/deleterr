use std::time::Duration;

use super::models::{APIData, APIResponse, RequestResponse};
use super::overseerr::{
    MediaRequest, OverseerrListResponse, OverseerrRequestsCount, OverseerrResponses,
};
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};

async fn make_api_call(endpoint: &str) -> Result<RequestResponse, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(format!("{os_request_url}{endpoint}"))
        .timeout(Duration::from_secs(15))
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    let request_response: RequestResponse = RequestResponse {
        code: response.status().as_u16(),
        status: response.status().to_string(),
        response,
    };

    Ok(request_response)
}

async fn map_to_response<T>(
    overseerr_response: OverseerrResponses<T>,
    code: u16,
    status: String,
) -> Result<APIResponse<T>, Error>
where
    for<'a> T: serde::Deserialize<'a>,
{
    if code != 200 {
        return Ok(APIResponse {
            success: false,
            data: APIData::Failure(status),
            code,
        });
    };

    let api_response = APIResponse {
        success: true,
        data: APIData::Success(overseerr_response),
        code,
    };

    Ok(api_response)
}

fn process_request<T>(requests: Result<APIResponse<T>, Error>) -> impl Responder
where
    T: serde::Serialize,
{
    return match requests {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            let err_response = APIResponse {
                success: false,
                data: APIData::<String>::Failure(error.to_string()).into(),
                code: 500,
            };
            return HttpResponse::Ok().json(err_response);
        }
    };
}

async fn map_requests_to_api_response(endpoint: &str) -> Result<APIResponse<MediaRequest>, Error> {
    let request_response = make_api_call(endpoint).await?;
    let resp = request_response
        .response
        .json::<OverseerrListResponse<MediaRequest>>()
        .await?;
    let overseerr_response: OverseerrResponses<MediaRequest> = OverseerrResponses::List(resp);
    let api_response = map_to_response(
        overseerr_response,
        request_response.code,
        request_response.status,
    )
    .await?;
    Ok(api_response)
}

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let count_response = map_requests_to_api_response(&endpoint).await;
    return process_request(count_response);
}

async fn map_count_to_api_response(endpoint: &str) -> Result<APIResponse<()>, Error> {
    let request_response = make_api_call(endpoint).await?;
    let resp = request_response
        .response
        .json::<OverseerrRequestsCount>()
        .await?;
    let overseerr_response: OverseerrResponses<()> = OverseerrResponses::Count(resp);
    let api_response = map_to_response(
        overseerr_response,
        request_response.code,
        request_response.status,
    )
    .await?;
    Ok(api_response)
}

#[get("/requests/count")]
async fn get_requests_count() -> impl Responder {
    let endpoint = "request/count";
    let count_response = map_count_to_api_response(&endpoint).await;
    return process_request(count_response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests).service(get_requests_count);
}
