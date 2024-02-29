use super::models::Movie;
use crate::common::models::api::{APIStatus, RequestType, ResponseCodeBasedAction};
use crate::common::models::deleterr_error::DeleterrError;
use crate::common::models::services::{ServiceInfo, Services};
use crate::common::services::{create_api_url, get_api_endpoint, make_api_call};

fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::services::get_service(Services::Radarr)?;

    service_info.ok_or(DeleterrError::new("Radarr service not setup."))
}

pub async fn get_movie(tmdb_id: &Option<usize>) -> Result<Option<Movie>, DeleterrError> {
    match tmdb_id {
        Some(tmdb_id) => {
            let id = tmdb_id.to_string();
            let endpoint = format!("api/v3/movie");
            let service_info = build_service_info()?;

            let api_url = create_api_url(&endpoint, &service_info);
            let query = vec![("tmdbId", id.as_str())];

            let client_req =
                get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;
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
                    Err(DeleterrError::from(error).add_prefix("Unable to process Radarr response."))
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
    let request_response = make_api_call(client_req).await;

    match request_response {
        Ok(_) => Ok(ResponseCodeBasedAction {
            status: APIStatus::Success,
            success: true,
        }),
        Err(error) => {
            Err(DeleterrError::from(error).add_prefix("Unable to process Radarr response."))
        }
    }
}

pub async fn get_cover(movie_id: usize) -> Result<Vec<u8>, DeleterrError> {
    let endpoint = format!("api/v3/mediacover/{movie_id}/poster.jpg");
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req).await;

    let resp = match request_response {
        Ok(req_resp) => Ok(req_resp.response.bytes().await?),
        Err(err) => Err(err),
    };

    match resp {
        Ok(img) => Ok(img.to_vec()),
        Err(error) => Err(DeleterrError::new(error.to_string().as_str())
            .add_prefix("Unable to get radarr image,")),
    }
}
