use std::{collections::HashMap, str};

use super::common::{get_collection, get_data, save_data};
use crate::common::models::{
    deleterr_error::DeleterrError,
    services::{ServiceInfo, Services},
};

const BUCKET_NAME: &str = "services";

pub fn get_service(service: Services) -> Result<Option<ServiceInfo>, DeleterrError> {
    let services_data = get_data(BUCKET_NAME, &service.to_string());

    match services_data {
        Ok(data) => {
            if let Some(serv) = data {
                let service_info: ServiceInfo = ServiceInfo::from(serv);
                Ok(Some(service_info))
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(DeleterrError::from(err).add_prefix("Unable to get service.")),
    }
}

pub fn get_all_services() -> Result<HashMap<String, ServiceInfo>, DeleterrError> {
    let mut all_services: HashMap<String, ServiceInfo> = HashMap::new();

    let collections = get_collection(BUCKET_NAME);

    match collections {
        Ok(services) => {
            for service in services {
                all_services.insert(service.0, ServiceInfo::from(service.1));
            }

            Ok(all_services)
        }
        Err(err) => Err(DeleterrError::from(err).add_prefix("Unable to get all services.")),
    }
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
