use crate::common::models::api::RequestType;
use crate::common::models::deleterr_error::DeleterrError;
use crate::common::models::services::{ServiceInfo, Services};
use crate::common::services::{create_api_url, get_api_endpoint, make_api_call};

use super::series::Series;

/// Retrieves information about the Sonarr service and returns it as a
/// `ServiceInfo` struct or an error if the service is not set up.
///
/// # Returns:
///
/// `Result` containing either a [crate::common::models::services::ServiceInfo] or a `DeleterrError`.
fn build_service_info() -> Result<ServiceInfo, DeleterrError> {
    let service_info = crate::store::services::services::get_service(Services::Sonarr)?;

    service_info.ok_or(DeleterrError::new("Sonarr service not setup."))
}

/// Retrieves a TV series based on its TVDB ID using the Sonarr API.
///
/// # Arguments:
///
/// * `tvdb_id`: An optional `usize` used to identify the series in Sonarr
///
/// # Returns:
///
/// `Result` containing an `Option` of `Series` or a `DeleterrError`. If a `Some` value is provided for the `tvdb_id`,
/// the function makes an API call to retrieve a list of series based on the provided TVDB ID.
/// If successful, it returns the first series in the list as an `Option`. If `None` value is provided for the `tbdb_id`
/// It returns a `Ok` with a `none` back
pub async fn get_series(tvdb_id: &Option<usize>) -> Result<Option<Series>, DeleterrError> {
    match tvdb_id {
        Some(tv_id) => {
            let endpoint = "api/v3/series".to_string();
            let service_info = build_service_info()?;
            let id = tv_id.to_string();

            let api_url = create_api_url(&endpoint, &service_info);
            let query = vec![("tvdbId", id.as_str())];

            let client_req =
                get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

            let request_response = make_api_call(client_req)
                .await
                .map_err(|err| err.add_prefix("Unable to get Sonarr show, "))?;

            let resp = request_response.json::<Vec<Series>>().await;

            match resp {
                Ok(series) => {
                    if series.len() > 0 {
                        Ok(series.get(0).cloned()) // TODO: Do we need to clone here?
                    } else {
                        Ok(None)
                    }
                }
                Err(error) => {
                    Err(DeleterrError::from(error)
                        .add_prefix("Unable to process Sonarr response, "))
                }
            }
        }
        None => Ok(None),
    }
}

/// Asynchronously retrieves a cover image for a given series ID using an API call.
///
/// # Arguments:
///
/// * `series_id`: Used to identify a specific series for which you want to
/// retrieve the cover image. It is a unique identifier assigned to each series by Sonarr.
///
/// # Returns:
///
/// `Result` containing a `Vec<u8>` representing the image data if
/// the operation is successful, or a `DeleterrError` if there was an error during the process.
pub async fn get_cover(series_id: usize) -> Result<Vec<u8>, DeleterrError> {
    let endpoint = format!("api/v3/mediacover/{series_id}/poster.jpg");
    let service_info = build_service_info()?;

    let api_url = create_api_url(&endpoint, &service_info);
    let query = Vec::with_capacity(0);

    let client_req =
        get_api_endpoint(api_url, query, Some(service_info.api_key), RequestType::Get)?;

    let request_response = make_api_call(client_req).await;

    let resp = match request_response {
        Ok(req_resp) => Ok(req_resp.bytes().await?),
        Err(err) => Err(err),
    };

    match resp {
        Ok(img) => Ok(img.to_vec()),
        Err(error) => Err(DeleterrError::new(error.to_string().as_str())
            .add_prefix("Unable to get sonarr image,")),
    }
}
