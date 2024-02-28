use std::{collections::HashMap, str};

use super::common::{get_collection, get_data, get_usize_keys, save_data};
use crate::common::models::{
    deleterr_error::DeleterrError, exemptions::MediaExemption, services::ServiceInfo,
};

const BUCKET_NAME: &str = "media_exemptions";

pub fn get_all_exemptions() -> Result<Vec<usize>, DeleterrError> {
    let mut media_exemptions: Vec<usize> = Vec::new();

    let collection = get_usize_keys(BUCKET_NAME).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save service.")
    });

    collection
}

pub fn upsert_service(service_info: ServiceInfo) -> Result<(), DeleterrError> {
    let service_upsert = save_data(
        &BUCKET_NAME,
        &service_info.as_vec(),
        &service_info.service.to_string(),
    )
    .map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save service.")
    });

    service_upsert
}
