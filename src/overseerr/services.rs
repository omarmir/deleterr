use std::time::Duration;

use super::models::{APIData, APIResponse};
use super::overseerr::{
    MediaRequest, OverseerrListResponse, OverseerrRequestsCount, OverseerrResponses,
    OverseerrResponsesTypes,
};
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
        .timeout(Duration::from_secs(15))
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json")
        .send()
        .await;

    return response;
}

async fn map_to_response<T>(
    response: Result<Response, Error>,
    resp_type: OverseerrResponsesTypes,
) -> Result<APIResponse<T>, Error>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let res = response?;
    let code = res.status().as_u16();

    if code != 200 {
        return Ok(APIResponse {
            success: false,
            data: APIData::Failure(res.text().await?).into(),
            code,
        });
    };

    let data = match resp_type {
        OverseerrResponsesTypes::List => APIData::Success(OverseerrResponses::List(
            res.json::<OverseerrListResponse<T>>().await?,
        )),
        OverseerrResponsesTypes::Count => APIData::Success(OverseerrResponses::Count(
            res.json::<OverseerrRequestsCount>().await?,
        )),
    };

    let api_response = APIResponse {
        success: true,
        data,
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

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let response = make_api_call(&endpoint).await;
    let requests = map_to_response::<MediaRequest>(response, OverseerrResponsesTypes::List).await;
    return process_request(requests);
}

#[get("/requests/count")]
async fn get_requests_count() -> impl Responder {
    let endpoint = "request/count";
    let response: Result<Response, Error> = make_api_call(&endpoint).await;

    return HttpResponse::Ok().json("sss");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests);
}
