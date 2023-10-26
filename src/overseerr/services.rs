use super::models::{
    AboutServer, MediaInfo, MediaRequest, MediaType, OverseerrListResponse, OverseerrRequestsCount,
};
use crate::common::models::{
    APIResponse, APIServiceStatus, APIStatus, DeleterrError, ServiceInfo, Services,
};
use crate::common::services::{
    create_api_url, get_api_endpoint, make_api_call, map_to_api_response,
};
use dotenv::dotenv;

fn build_service_info() -> ServiceInfo {
    dotenv().ok();
    let os_host = std::env::var("OS_HOST").expect("os_host must be set.");
    let os_api_key = std::env::var("OS_API_KEY").expect("os_api_key must be set.");
    let os_port = std::env::var("OS_PORT").expect("os_port must be set.");
    let os_use_ssl = std::env::var("OS_USE_SSL").expect("os_use_ssl must be set.");

    let parsed_port = os_port.parse::<String>().ok();

    return ServiceInfo {
        host: os_host,
        api_key: os_api_key,
        port: parsed_port,
        use_ssl: if os_use_ssl == "true" { true } else { false },
        service: Services::Overseerr,
    };
}

pub async fn get_requests(
    take: &str,
    skip: &str,
) -> Result<APIResponse<OverseerrListResponse<MediaRequest>>, DeleterrError> {
    let endpoint = format!("api/v1/request");
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![
        ("take", take),
        ("skip", skip),
        ("sort", "added"),
        ("filter", "available"),
    ];
    let client_req = get_api_endpoint(api_url, query, Some(service_info.api_key))?;

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
    let endpoint = "api/v1/request/count".to_string();
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req = get_api_endpoint(api_url, query, Some(service_info.api_key))?;
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
        MediaType::TV => format!("api/v1/tv/{id}"),
        MediaType::Movie => format!("api/v1/movie/{id}"),
    };
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![("sort", "added"), ("filter", "available")];

    let client_req = get_api_endpoint(api_url, query, Some(service_info.api_key))?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<MediaInfo>().await?;

    Ok(resp)
}

pub async fn get_overseerr_status(
    service_info: ServiceInfo,
) -> Result<APIResponse<APIServiceStatus>, DeleterrError> {
    let endpoint: String = "api/v1/settings/about".to_string();

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req = get_api_endpoint(api_url, query, Some(service_info.api_key))?;

    let request_response = make_api_call(client_req).await?;
    // We need to make sure its actaully the response from Overseer and not just an OK response
    let resp = request_response.response.json::<AboutServer>().await;

    //This is a nested match which is a bit messy but the if let statements were harder to parse mentally
    let service_status = match resp {
        Ok(_) => APIServiceStatus {
            status: APIStatus::Success,
            service: Services::Overseerr,
            is_success: true,
        },
        _ => match &request_response.code {
            200 => APIServiceStatus {
                status: APIStatus::Success,
                service: Services::Overseerr,
                is_success: true,
            },
            403 => APIServiceStatus {
                status: APIStatus::WrongAPIKey,
                service: Services::Overseerr,
                is_success: false,
            },
            _ => APIServiceStatus {
                status: APIStatus::Other,
                service: Services::Overseerr,
                is_success: false,
            },
        },
    };

    let api_response = map_to_api_response(service_status, 200, "Failure".to_string()).await?;
    Ok(api_response)
}
