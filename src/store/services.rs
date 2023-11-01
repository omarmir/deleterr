#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, path::Path};

use persy::{Config, OpenError, Persy, PersyId, ValueMode, PE};

use crate::common::models::{DeleterrError, ServiceInfo, Services};

pub fn get_store() -> Result<Persy, PE<OpenError>> {
    let path = Path::new("deleterr.persy");
    let config = Config::new();

    Persy::open_or_create_with(path, config, |persy| {
        // this closure is only called on database creation
        let mut tx = persy.begin()?;
        tx.create_segment("services")?;
        tx.create_index::<String, PersyId>("index", ValueMode::Cluster)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        println!("Segment and Index successfully created");
        Ok(())
    })
}

pub fn does_record_exist(
    persy: &Persy,
    service_name: &Services,
) -> Result<Option<PersyId>, DeleterrError> {
    let read_id = persy
        .get::<String, PersyId>("index", &service_name.to_string())?
        .next();

    if let Some(id) = read_id {
        Ok(Some(id))
    } else {
        Ok(None)
    }
}

pub fn upsert_service(service_info: ServiceInfo) -> Result<String, DeleterrError> {
    let persy = get_store()?;
    //Start a transaction all the operations in persy are done inside a transaction.
    let persy_id = does_record_exist(&persy, &service_info.service)?;

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
            let new_id = tx.insert("services", rec)?;

            tx.put::<String, PersyId>("index", service_info.service.to_string(), new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
    }
}

pub fn get_service(service_name: Services) -> Result<Option<ServiceInfo>, DeleterrError> {
    let persy = get_store()?;

    let read_id = persy
        .get::<String, PersyId>("index", &service_name.to_string())?
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

    for (read_id, content) in persy.scan("services")? {
        let service = ServiceInfo::from(content);
        all_services.insert(service.service.to_string(), Some(service));
    }

    Ok(all_services)
}
