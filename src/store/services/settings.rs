use super::common::{get_collection, save_multiple_items};
use crate::common::models::deleterr_error::DeleterrError;

const BUCKET_NAME: &str = "settings";

pub fn get_all_services() -> Result<Vec<(String, Vec<u8>)>, DeleterrError> {
    let collection = get_collection(BUCKET_NAME).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all settings.")
    });

    collection
}

pub fn upsert_settings(settings: Vec<(&str, &str)>) -> Result<(), DeleterrError> {
    let service_upsert = save_multiple_items(&BUCKET_NAME, settings).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save media exemption.")
    });

    service_upsert
}
