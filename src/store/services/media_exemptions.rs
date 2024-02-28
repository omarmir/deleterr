use super::common::{get_usize_keys, remove_pair, save_usize_keys_only};
use crate::{common::models::deleterr_error::DeleterrError, store::models::EitherKeyType};

const BUCKET_NAME: &str = "media_exemptions";

pub fn get_all_exemptions() -> Result<Vec<usize>, DeleterrError> {
    let collection = get_usize_keys(BUCKET_NAME).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all exemptions.")
    });

    collection
}

pub fn upsert_media_exemption(media_exemption: usize) -> Result<(), DeleterrError> {
    let service_upsert = save_usize_keys_only(&BUCKET_NAME, &media_exemption).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save media exemption.")
    });

    service_upsert
}

pub fn remove_media_exemption(request_id: usize) -> Result<bool, DeleterrError> {
    let deletion = remove_pair(&BUCKET_NAME, EitherKeyType::Number(request_id)).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to remove media exemption.")
    });

    deletion
}
