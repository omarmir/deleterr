use super::models::TautulliResponse;
use super::user_watch_history::UserWatchHistory;
use crate::common::models::{APIServiceStatus, APIStatus, RequestType, Services};
use crate::common::models::{DeleterrError, ServiceInfo};
use crate::common::services::{create_api_url, get_api_endpoint, make_api_call};
use crate::overseerr::models::MediaType;

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::get_service(Services::Tautulli)?;

    service_info.ok_or(DeleterrError::new("Tautulli service not setup."))
}

pub async fn get_item_history(
    rating_key: &Option<usize>,
    user_id: &Option<usize>,
    media_type: &MediaType,
) -> Result<Option<Vec<UserWatchHistory>>, DeleterrError> {
    match (rating_key, user_id) {
        (Some(rating_key), Some(user_id)) => {
            let endpoint = format!("api/v2");
            let service_info = build_service_info()?;

            let api_url = create_api_url(&endpoint, &service_info);
            let rk = rating_key.to_string();
            let uid = user_id.to_string();

            let rating_key_lvl = match media_type {
                MediaType::Movie => ("rating_key", rk.as_str()),
                MediaType::TV => ("grandparent_rating_key", rk.as_str()),
            };

            let query = vec![
                ("cmd", "get_history"),
                ("apikey", service_info.api_key.as_str()),
                ("user_id", uid.as_str()),
                ("length", "99999999999999999"),
                rating_key_lvl,
            ];

            let client_req = get_api_endpoint(api_url, query, None, RequestType::Get)?;
            let request_response = make_api_call(client_req).await?;
            let resp = request_response.response.json::<TautulliResponse>().await?;
            Ok(resp.response.data.data)
        }
        _ => Ok(None),
    }
}

pub async fn get_tautulli_status(
    service_info: ServiceInfo,
) -> Result<APIServiceStatus, DeleterrError> {
    let endpoint = format!("api/v2");
    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![("cmd", "status"), ("apikey", service_info.api_key.as_str())];
    let client_req = get_api_endpoint(api_url, query, None, RequestType::Get)?;
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
                    status: APIStatus::WrongKey,
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

    Ok(service_status)
}
