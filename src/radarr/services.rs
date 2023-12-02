use super::models::Movie;
use crate::{
    common::models::ResponseCodeBasedAction,
    common::{
        models::{APIStatus, DeleterrError, RequestType, ServiceInfo, Services},
        services::{create_api_url, get_api_endpoint, make_api_call},
    },
};

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::get_service(Services::Radarr)?;

    service_info.ok_or(DeleterrError::new("Radarr service not setup."))
}

pub async fn get_movie(tmdb_id: &Option<usize>) -> Result<Option<Movie>, DeleterrError> {
    match tmdb_id {
        Some(tmdb_id) => {
            let endpoint = format!("api/v3/movie/{tmdb_id}");
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

            let resp = request_response.response.json::<Vec<Movie>>().await;

            match resp {
                Ok(movie) => {
                    if movie.len() > 0 {
                        Ok(movie.get(0).cloned())
                    } else {
                        Ok(None)
                    }
                }
                Err(error) => {
                    Err(DeleterrError::from(error).add_prefix("Unable to process Radarr response,"))
                }
            }
        }
        None => Ok(None),
    }
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
