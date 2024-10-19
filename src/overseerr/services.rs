use super::models::{
    AboutServer, MediaRequest, OverseerrListResponse, OverseerrRequestsCount, PageInfo, RadarrInfo,
};
use crate::common::models::api::{
    APIServiceStatus, APIStatus, RequestType, ResponseCodeBasedAction,
};
use crate::common::models::deleterr_error::DeleterrError;
use crate::common::models::services::{ServiceInfo, Services};
use crate::common::services::{create_api_url, get_api_endpoint, make_api_call};

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::services::get_service(Services::Overseerr)?;

    service_info.ok_or(DeleterrError::new("Overseerr service not setup."))
}

pub async fn get_requests() -> Result<OverseerrListResponse<MediaRequest>, DeleterrError> {
    let endpoint = format!("api/v1/request");
    let service_info = build_service_info()?;

    let total_count = get_requests_count().await?.available.to_string();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![
        ("sort", "added"),
        ("filter", "available"),
        ("take", total_count.as_str()),
    ];
    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req)
        .await
        .map_err(|err| err.add_prefix("Unable to get Overseerr requests."))?;
    let resp = request_response
        .json::<OverseerrListResponse<MediaRequest>>()
        .await?;

    Ok(resp)
}

pub async fn get_requests_count() -> Result<OverseerrRequestsCount, DeleterrError> {
    let endpoint = "api/v1/request/count".to_string();
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;
    let request_response = make_api_call(client_req)
        .await
        .map_err(|err| err.add_prefix("Unable to get Overseerr request count."))?;
    let resp = request_response.json::<OverseerrRequestsCount>().await?;

    Ok(resp)
}

pub async fn delete_media(media_id: &str) -> Result<ResponseCodeBasedAction, DeleterrError> {
    let endpoint = format!("api/v1/media/{media_id}");
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req = get_api_endpoint(
        api_url,
        query,
        Some(service_info.api_key),
        RequestType::Delete,
    )?;
    let request_response = make_api_call(client_req).await;

    match request_response {
        Ok(_) => Ok(ResponseCodeBasedAction {
            status: APIStatus::Success,
            success: true,
        }),
        Err(error) => Err(error.add_prefix("Unable to delete request in Overseerr.")),
    }
}

pub async fn get_overseerr_radar_info() -> Result<Option<ServiceInfo>, DeleterrError> {
    let endpoint = format!("api/v1/settings/radarr");
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req)
        .await
        .map_err(|err| err.add_prefix("Unable to get radarr info from overseerr."))?;

    let resp = request_response
        .json::<Vec<RadarrInfo>>()
        .await
        .map_err(|_err| DeleterrError::new("Unable to parse radarr info from overseerr."))?;

    if resp.len() > 0 {
        let radarr_info = resp[0].clone();

        let radarr_service_info = ServiceInfo {
            host: radarr_info.hostname,
            port: radarr_info.port,
            api_key: radarr_info.api_key,
            use_ssl: radarr_info.use_ssl,
            service: Services::Radarr,
        };

        Ok(Some(radarr_service_info))
    } else {
        Ok(None)
    }
}

pub async fn get_overseerr_status(
    service_info: ServiceInfo,
) -> Result<APIServiceStatus, DeleterrError> {
    let endpoint: String = "api/v1/settings/about".to_string();

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req)
        .await
        .map_err(|err| err.add_prefix("Unable to get Overseerr status."))?;

    // We need to make sure its actaully the response from Overseer and not just an OK response
    let resp = request_response.json::<AboutServer>().await;

    //This is a nested match which is a bit messy but the if let statements were harder to parse mentally
    match resp {
        Ok(_) => Ok(APIServiceStatus {
            status: APIStatus::Success,
            service: Services::Overseerr,
            is_success: true,
        }),
        Err(error) => {
            Err(DeleterrError::from(error).add_prefix("Unable to process Overseerr response, "))
        }
    }
}

pub async fn get_os_requests() -> Result<(Vec<MediaRequest>, PageInfo), DeleterrError> {
    let os_requests = get_requests().await?;
    let vec_requests = os_requests.results;
    let page_info = os_requests.page_info;
    Ok((vec_requests, page_info))
}
