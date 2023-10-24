use super::models::TautulliResponse;
use crate::common::models::{APIResponse, DeleterrError};
use crate::common::models::{APIServiceStatus, APIStatus, Services};
use crate::common::services::{make_api_call, map_to_api_response};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};
use std::time::Duration;

fn get_api_endpoint(endpoint: String) -> Result<reqwest::RequestBuilder, Error> {
    dotenv().ok();

    let client = reqwest::Client::new();

    let tt_request_url = std::env::var("TT_REQUEST_URL").expect("TT_request_url must be set.");
    let tt_api_key = std::env::var("TT_API_KEY").expect("TT_api_key must be set.");

    let req_client = client
        .get(format!("{tt_request_url}{tt_api_key}{endpoint}"))
        .timeout(Duration::from_secs(15))
        .header(ACCEPT, "application/json");

    Ok(req_client)
}

pub async fn get_item_history(
    rating_key: u64,
    user_id: u64,
) -> Result<APIResponse<TautulliResponse>, DeleterrError> {
    let endpoint = format!("&cmd=get_history&rating_key={rating_key}&user_id={user_id}");
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;
    let api_response =
        map_to_api_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

pub async fn get_tautulli_status() -> Result<APIResponse<APIServiceStatus>, DeleterrError> {
    let endpoint = format!("&cmd=status");
    let client_req = get_api_endpoint(endpoint)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;

    //This is a nested match which is a bit messy but the if let statements were harder to parse mentally
    let service_status = match resp.response.result {
        super::models::Result::Success => APIServiceStatus {
            status: APIStatus::Success,
            service: Services::Tautulli,
            is_success: true,
        },
        _ => match resp.response.message {
            Some(msg) => match msg.as_str() {
                "Invalid apikey" => APIServiceStatus {
                    status: APIStatus::WrongAPIKey,
                    service: Services::Tautulli,
                    is_success: false,
                },
                _ => APIServiceStatus {
                    status: APIStatus::Other,
                    service: Services::Overseer,
                    is_success: false,
                },
            },
            _ => APIServiceStatus {
                status: APIStatus::Other,
                service: Services::Overseer,
                is_success: false,
            },
        },
    };

    let api_response = map_to_api_response(service_status, 200, "Failure".to_string()).await?;
    Ok(api_response)
}
