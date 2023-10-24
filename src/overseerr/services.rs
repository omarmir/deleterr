use super::models::{
    MediaInfo, MediaRequest, MediaType, OverseerrListResponse, OverseerrRequestsCount,
};
use crate::common::models::{APIResponse, APIServiceStatus, APIStatus, DeleterrError, Services};
use crate::common::services::{make_api_call, map_to_api_response};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};
use std::time::Duration;

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
    take: usize,
    skip: usize,
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

pub async fn get_requests_count() -> Result<APIResponse<OverseerrRequestsCount>, DeleterrError> {
    let endpoint: String = "request/count".to_string();
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

pub async fn get_media_info(
    media_type: &MediaType,
    id: &usize,
) -> Result<MediaInfo, DeleterrError> {
    let endpoint: String = match media_type {
        MediaType::TV => format!("tv/{id}"),
        MediaType::Movie => format!("movie/{id}"),
    };
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<MediaInfo>().await?;

    Ok(resp)
}

pub async fn get_overseerr_status() -> Result<APIResponse<APIServiceStatus>, DeleterrError> {
    let endpoint: String = "settings/about".to_string();
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;

    //This is a nested match which is a bit messy but the if let statements were harder to parse mentally
    let service_status = match request_response.code {
        200 => APIServiceStatus {
            status: APIStatus::Success,
            service: Services::Overseer,
            is_success: true,
        },
        403 => APIServiceStatus {
            status: APIStatus::WrongAPIKey,
            service: Services::Overseer,
            is_success: false,
        },
        _ => APIServiceStatus {
            status: APIStatus::Other,
            service: Services::Overseer,
            is_success: false,
        },
    };

    let api_response = map_to_api_response(service_status, 200, "Failure".to_string()).await?;
    Ok(api_response)
}
