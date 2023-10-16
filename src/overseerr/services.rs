use super::models::{APIData, APIResponse};
use super::overseerr::{MediaRequest, OverseerrListResponse};
use actix_web::{get, web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error, Response};

async fn make_api_call<T: serde::de::DeserializeOwned>(endpoint: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    dotenv().ok();
    let os_request_url = std::env::var("OS_REQUEST_URL").expect("os_request_url must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");

    let response = client
        .get(format!("{os_request_url}{endpoint}"))
        .header("X-Api-Key", os_api_key)
        .header(ACCEPT, "application/json")
        .send()
        .await;

    return response;
}

async fn map_to_response<T>(response: Result<Response, Error>) -> Result<APIResponse<T>, Error> {
    return match response {
        Ok(res) => {
            let response_code = res.status().as_u16();
            let list_response = res.json::<OverseerrListResponse<T>>().await?;
            let api_response = APIResponse {
                success: true,
                data: APIData::Success(list_response),
                code: response_code,
            };
            Ok(api_response)
        }
        Err(err) => Ok(APIResponse {
            success: false,
            data: APIData::Failure(err.to_string()),
            code: 500,
        }),
    };
}

fn process_request<T>(requests: Result<APIResponse<T>, Error>) -> impl Responder
where
    T: serde::Serialize,
{
    return match requests {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    };
}

#[get("/requests/all")]
async fn get_requests() -> impl Responder {
    let endpoint = "request?take=20&skip=0&sort=added&filter=available";
    let response_result: Result<APIResponse<MediaRequest>, Error> = make_api_call(&endpoint).await;
    //let requests = get_requests_response().await;
    return process_request(response_result);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_requests);
}
