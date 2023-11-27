use crate::common::{
    models::{DeleterrError, RequestType, ServiceInfo, Services},
    services::{create_api_url, get_api_endpoint, make_api_call},
};

use super::models::Episode;

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::get_service(Services::Sonarr)?;

    service_info.ok_or(DeleterrError::new("Sonarr service not setup."))
}

pub async fn get_episodes(series_id: usize) -> Result<Vec<Episode>, DeleterrError> {
    let endpoint = format!("api/v3/episode?seriesId={series_id}");
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

    let resp = request_response.response.json::<Vec<Episode>>().await?;

    Ok(resp)
}
