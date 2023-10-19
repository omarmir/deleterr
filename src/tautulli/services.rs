use super::models::TautulliResponse;
use crate::deleterr::models::{APIData, APIResponse, RequestResponse};
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};
use std::time::Duration;

async fn make_api_call(endpoint: &str) -> Result<RequestResponse, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let tt_request_url = std::env::var("TT_REQUEST_URL").expect("os_request_url must be set.");
    let tt_api_key = std::env::var("TT_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(format!("{tt_request_url}{tt_api_key}{endpoint}"))
        .timeout(Duration::from_secs(15))
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
    overseerr_response: TautulliResponse,
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
        data: APIData::TautulliSuccess(overseerr_response),
        code,
    };

    Ok(api_response)
}

async fn map_requests_to_api_response(
    endpoint: &str,
) -> Result<APIResponse<TautulliResponse>, Error> {
    let request_response = make_api_call(endpoint).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;
    let api_response =
        map_to_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

pub async fn get_item_history() -> Result<APIResponse<TautulliResponse>, Error> {
    let endpoint = "&cmd=get_history&rating_key=29946";
    let requests_response = map_requests_to_api_response(&endpoint).await;
    return requests_response;
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
                data: APIData::<String>::Failure(error.to_string()),
                code: 500,
            };
            return HttpResponse::Ok().json(err_response);
        }
    };
}

#[get("/history")]
async fn get_requests_json() -> impl Responder {
    let requests_response = get_item_history().await;
    return process_request(requests_response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests_json);
}
