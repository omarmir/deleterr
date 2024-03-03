use super::common::get_collection;
use crate::common::models::deleterr_error::DeleterrError;

const BUCKET_NAME: &str = "preferences";

pub fn get_all_services() -> Result<Vec<(String, Vec<u8>)>, DeleterrError> {
    let collection = get_collection(BUCKET_NAME).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all settings.")
    });

    collection
}

pub fn upsert_settings(settings: Vec<(&str, &str)>) -> Result<(), DeleterrError> {
    unimplemented!()
}
