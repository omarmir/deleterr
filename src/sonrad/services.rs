use super::models::SonRadStatus;
use crate::common::models::api::{APIServiceStatus, APIStatus, RequestType};
use crate::common::models::deleterr_error::DeleterrError;
use crate::common::models::services::{ServiceInfo, Services};
use crate::common::services::{create_api_url, get_api_endpoint, make_api_call};

pub async fn get_sonrad_status(
    service_info: ServiceInfo,
) -> Result<APIServiceStatus, DeleterrError> {
    let endpoint = format!("api/v3/system/status");
    let api_url = create_api_url(&endpoint, &service_info);
    let query: Vec<(&str, &str)> = Vec::with_capacity(0);
    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req).await?;
    // We need to make sure its actaully the response from Radarr/Sonarr and not just an OK response
    let resp = request_response.response.json::<SonRadStatus>().await;

    let service_status = match resp {
        Ok(rd_resp) => {
            if rd_resp.app_name == service_info.service.to_name() {
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
