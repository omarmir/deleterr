use std::{collections::HashMap, str};

use super::common::{get_collection, get_data, save_data};
use crate::common::models::{
    deleterr_error::DeleterrError, exemptions::MediaExemption, services::ServiceInfo,
};

const BUCKET_NAME: &str = "media_exemptions";

pub fn get_all_exemptions() -> Result<Vec<usize>, DeleterrError> {
    let mut media_exemptions: Vec<usize> = Vec::new();

    let collection = get_collection(BUCKET_NAME).unwrap_or(Vec::new());

    for exemption in collection {
        let req_id = exemption.0.parse::<usize>();
        media_exemptions.push(exemption.0.parse::<usize>());
    }

    Ok(media_exemptions)
}
pub fn get_all_services() -> Result<HashMap<String, ServiceInfo>, DeleterrError> {
    let mut all_services: HashMap<String, ServiceInfo> = HashMap::new();

    let collections = get_collection(BUCKET_NAME).unwrap_or(Vec::new());

    for service in collections {
        all_services.insert(service.0, ServiceInfo::from(service.1));
    }

    Ok(all_services)
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
