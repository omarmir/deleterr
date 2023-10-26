use super::models::TautulliResponse;
use crate::common::models::{APIResponse, DeleterrError, ServiceInfo};
use crate::common::models::{APIServiceStatus, APIStatus, Services};
use crate::common::services::{create_api_url, make_api_call, map_to_api_response};
use dotenv::dotenv;
use reqwest::{header::ACCEPT, Error};
use std::time::Duration;

fn get_api_endpoint(
    url: String,
    query: Vec<(&str, &str)>,
) -> Result<reqwest::RequestBuilder, Error> {
    let req_client = reqwest::Client::new()
        .get(url)
        .query(&query)
        .timeout(Duration::from_secs(15))
        .header(ACCEPT, "application/json");

    Ok(req_client)
}

fn build_service_info() -> ServiceInfo {
    dotenv().ok();
    let os_host = std::env::var("TT_HOST").expect("tt_host must be set.");
    let os_api_key = std::env::var("TT_API_KEY").expect("tt_api_key must be set.");
    let os_port = std::env::var("TT_PORT").expect("tt_port must be set.");
    let os_use_ssl = std::env::var("TT_USE_SSL").expect("tt_use_ssl must be set.");

    let parsed_port = os_port.parse::<String>().ok();

    return ServiceInfo {
        host: os_host,
        api_key: os_api_key,
        port: parsed_port,
        use_ssl: if os_use_ssl == "true" { true } else { false },
        service: Services::Overseerr,
    };
}

pub async fn get_item_history(
    rating_key: &str,
    user_id: &str,
) -> Result<APIResponse<TautulliResponse>, DeleterrError> {
    let endpoint = format!("api/v2");
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![
        ("cmd", "get_history"),
        ("rating_key", rating_key),
        ("user_id", user_id),
    ];

    let client_req = get_api_endpoint(api_url, query)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;
    let api_response =
        map_to_api_response(resp, request_response.code, request_response.status).await?;
    Ok(api_response)
}

pub async fn get_tautulli_status() -> Result<APIResponse<APIServiceStatus>, DeleterrError> {
    let endpoint = format!("api/v2");
    let service_info = build_service_info();
    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![("cmd", "status")];
    let client_req = get_api_endpoint(api_url, query)?;
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
                    service: Services::Tautulli,
                    is_success: false,
                },
            },
            _ => APIServiceStatus {
                status: APIStatus::Other,
                service: Services::Tautulli,
                is_success: false,
            },
        },
    };

    let api_response = map_to_api_response(service_status, 200, "Failure".to_string()).await?;
    Ok(api_response)
}
