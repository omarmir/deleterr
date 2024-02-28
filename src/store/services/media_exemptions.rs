use super::common::{get_usize_keys, remove_pair, save_data};
use crate::common::models::{deleterr_error::DeleterrError, exemptions::MediaExemption};

const BUCKET_NAME: &str = "media_exemptions";

pub fn get_all_exemptions() -> Result<Vec<usize>, DeleterrError> {
    let collection = get_usize_keys(BUCKET_NAME).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all exemptions.")
    });

    collection
}

pub fn upsert_media_exemption(media_exemption: MediaExemption) -> Result<(), DeleterrError> {
    let service_upsert = save_data(
        &BUCKET_NAME,
        &media_exemption.as_vec(),
        &media_exemption.request_id.to_string(),
    )
    .map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save media exemption.")
    });

    service_upsert
}

pub fn remove_media_exemption(request_id: usize) -> Result<bool, DeleterrError> {
    let deletion = remove_pair(&BUCKET_NAME, request_id.to_string().as_str()).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to remove media exemption.")
    });

    deletion
}
