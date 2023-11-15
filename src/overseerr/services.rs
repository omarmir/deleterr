use super::models::{
    AboutServer, MediaInfo, MediaRequest, MediaType, OverseerrListResponse, OverseerrRequestsCount,
};
use crate::common::{
    models::{
        APIServiceStatus, APIStatus, DeleterrError, RequestType, ResponseCodeBasedAction,
        ServiceInfo, Services,
    },
    services::{create_api_url, get_api_endpoint, make_api_call},
};

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::st_serv::get_service(Services::Overseerr)?;

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

    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
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
    let request_response = make_api_call(client_req).await?;
    let resp = request_response
        .response
        .json::<OverseerrRequestsCount>()
        .await?;

    Ok(resp)
}

pub async fn get_media_info(
    media_type: &MediaType,
    tmdb_id: &Option<usize>,
) -> Result<MediaInfo, DeleterrError> {
    match tmdb_id {
        Some(id) => {
            let endpoint: String = match media_type {
                MediaType::TV => format!("api/v1/tv/{id}"),
                MediaType::Movie => format!("api/v1/movie/{id}"),
            };
            let service_info = build_service_info()?;

            let api_url = create_api_url(&endpoint, &service_info);
            let query = vec![];

            let client_req =
                get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;
            let request_response = make_api_call(client_req).await?;
            let resp = request_response.response.json::<MediaInfo>().await?;

            Ok(resp)
        }
        None => Ok({
            MediaInfo {
                title: "Unknown".to_string(),
                poster_path: None,
                release_date: None,
            }
        }),
    }
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
    let request_response = make_api_call(client_req).await?;
    match request_response.code {
        204 => {
            return Ok(ResponseCodeBasedAction {
                status: APIStatus::Success,
                success: true,
            })
        }
        404 => return Err(DeleterrError::new(APIStatus::NotFound.as_str())),
        403 => return Err(DeleterrError::new(APIStatus::WrongKey.as_str())),
        _ => return Err(DeleterrError::new(APIStatus::Other.as_str())),
    };
}

pub async fn get_overseerr_status(
    service_info: ServiceInfo,
) -> Result<APIServiceStatus, DeleterrError> {
    let endpoint: String = "api/v1/settings/about".to_string();

    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

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
                status: APIStatus::WrongKey,
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

    Ok(service_status)
}
