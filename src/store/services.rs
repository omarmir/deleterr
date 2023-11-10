use super::store::{does_record_exist, get_store};
use crate::common::models::{DeleterrError, ServiceInfo, Services};
use persy::PersyId;
use std::collections::HashMap;

pub fn upsert_service(service_info: ServiceInfo) -> Result<String, DeleterrError> {
    let persy = get_store()?;
    //Start a transaction all the operations in persy are done inside a transaction.
    let persy_id = does_record_exist(&persy, &service_info.service, "services_index")?;

    let mut tx = persy.begin()?;
    let rec = &service_info.as_bytes();
    match persy_id {
        Some(id) => {
            tx.update("services", &id, &rec)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(id.to_string())
        }
        None => {
            let new_id = tx.insert("services", &rec)?;

            tx.put::<String, PersyId>("services_index", service_info.service.to_string(), new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
    }
}

pub fn get_service(service_name: Services) -> Result<Option<ServiceInfo>, DeleterrError> {
    let persy = get_store()?;

    let read_id = persy
        .get::<String, PersyId>("services_index", &service_name.to_string())?
        .next();

    if let Some(id) = read_id {
        let value = persy.read("services", &id)?;
        match value {
            Some(val) => Ok(Some(ServiceInfo::from(val))),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn get_all_services() -> Result<HashMap<String, Option<ServiceInfo>>, DeleterrError> {
    let persy = get_store()?;
    // TODO: once we have the other services this needs to be updated
    let mut all_services: HashMap<String, Option<ServiceInfo>> = HashMap::from([
        (Services::Overseerr.to_string(), None),
        (Services::Tautulli.to_string(), None),
        ("radarr".to_string(), None),
        ("sonarr".to_string(), None),
    ]);

    for (_read_id, content) in persy.scan("services")? {
        let service = ServiceInfo::from(content);
        all_services.insert(service.service.to_string(), Some(service));
    }

    Ok(all_services)
}
