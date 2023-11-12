use crate::{
    common::models::ResponseCodeBasedAction,
    common::{
        models::{APIStatus, DeleterrError, RequestType, ServiceInfo, Services},
        services::{create_api_url, get_api_endpoint, make_api_call},
    },
};
use dotenv::dotenv;

fn build_service_info() -> ServiceInfo {
    dotenv().ok();
    let os_host = std::env::var("RD_HOST").expect("RD_host must be set.");
    let os_api_key = std::env::var("RD_API_KEY").expect("RD_api_key must be set.");
    let os_port = std::env::var("RD_PORT").expect("RD_port must be set.");
    let os_use_ssl = std::env::var("RD_USE_SSL").expect("RD_use_ssl must be set.");

    let parsed_port = os_port.parse::<String>().ok();

    return ServiceInfo {
        host: os_host,
        api_key: os_api_key,
        port: parsed_port,
        use_ssl: if os_use_ssl == "true" { true } else { false },
        service: Services::Radarr,
    };
}

pub async fn delete_movie(radarr_id: &str) -> Result<ResponseCodeBasedAction, DeleterrError> {
    let endpoint = format!("api/v3/movie/{radarr_id}");
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![("addImportExclusion", "false"), ("deleteFiles", "true")];

    let client_req = get_api_endpoint(
        api_url,
        query,
        Some(service_info.api_key),
        RequestType::Delete,
    )?;
    let request_response = make_api_call(client_req).await?;
    let resp = match request_response.code {
        200 => ResponseCodeBasedAction {
            status: APIStatus::Success,
            is_success: true,
        },
        404 => ResponseCodeBasedAction {
            status: APIStatus::NotFound,
            is_success: false,
        },
        401 => ResponseCodeBasedAction {
            status: APIStatus::WrongAPIKey,
            is_success: false,
        },
        _ => ResponseCodeBasedAction {
            status: APIStatus::Other,
            is_success: false,
        },
    };

    Ok(resp)
}
