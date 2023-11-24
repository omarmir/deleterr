use crate::{
    common::models::ResponseCodeBasedAction,
    common::{
        models::{APIServiceStatus, APIStatus, DeleterrError, RequestType, ServiceInfo, Services},
        services::{create_api_url, get_api_endpoint, make_api_call},
    },
};

use super::models::RadarrStatus;

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::st_serv::get_service(Services::Radarr)?;

    service_info.ok_or(DeleterrError::new("Radarr service not setup."))
}

pub async fn get_radarr_status(
    service_info: ServiceInfo,
) -> Result<APIServiceStatus, DeleterrError> {
    let endpoint = format!("api/v3/system/status");
    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);
    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req).await?;
    // We need to make sure its actaully the response from Radarr and not just an OK response
    let resp = request_response.response.json::<RadarrStatus>().await;

    let service_status = match resp {
        Ok(rd_resp) => {
            if rd_resp.app_name == String::from("Radarr") {
                APIServiceStatus {
                    status: APIStatus::Success,
                    service: Services::Radarr,
                    is_success: true,
                }
            } else {
                APIServiceStatus {
                    status: APIStatus::NotFound,
                    service: Services::Radarr,
                    is_success: false,
                }
            }
        }
        _ => match &request_response.code {
            401 => APIServiceStatus {
                status: APIStatus::WrongKey,
                service: Services::Radarr,
                is_success: false,
            },
            _ => APIServiceStatus {
                status: APIStatus::Other,
                service: Services::Radarr,
                is_success: false,
            },
        },
    };

    Ok(service_status)
}

pub async fn delete_movie(radarr_id: &str) -> Result<ResponseCodeBasedAction, DeleterrError> {
    let endpoint = format!("api/v3/movie/{radarr_id}");
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![("addImportExclusion", "false"), ("deleteFiles", "true")];

    let client_req = get_api_endpoint(
        api_url,
        query,
        Some(service_info.api_key),
        RequestType::Delete,
    )?;
    let request_response = make_api_call(client_req).await?;
    match request_response.code {
        200 => {
            return Ok(ResponseCodeBasedAction {
                status: APIStatus::Success,
                success: true,
            })
        }
        404 => return Err(DeleterrError::new(APIStatus::NotFound.to_string().as_str())),
        401 => return Err(DeleterrError::new(APIStatus::WrongKey.to_string().as_str())),
        _ => return Err(DeleterrError::new(APIStatus::Other.to_string().as_str())),
    };
}
