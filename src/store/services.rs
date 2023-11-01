#![allow(dead_code)]
#![allow(unused_variables)]

use std::path::Path;

use persy::{Config, Persy, PersyId, ValueMode};

use crate::common::models::{ServiceInfo, Services};

pub fn get_store() -> Persy {
    let path = Path::new("deleterr.persy");
    let config = Config::new();

    let store = Persy::open_or_create_with(path, config, |persy| {
        // this closure is only called on database creation
        let mut tx = persy.begin()?;
        tx.create_segment("services")?;
        tx.create_index::<String, PersyId>("index", ValueMode::Cluster)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        println!("Segment and Index successfully created");
        Ok(())
    })
    .unwrap();

    return store;
}

pub fn does_record_exist(persy: &Persy, service_name: &Services) -> Option<PersyId> {
    let read_id = persy
        .get::<String, PersyId>("index", &service_name.to_string())
        .unwrap()
        .next();

    if let Some(id) = read_id {
        Some(id)
    } else {
        None
    }
}

pub fn upsert_service(service_info: ServiceInfo) -> PersyId {
    let persy = get_store();
    //Start a transaction all the operations in persy are done inside a transaction.
    let persy_id = does_record_exist(&persy, &service_info.service);

    let mut tx = persy.begin().unwrap();
    let rec = &service_info.as_bytes();
    match persy_id {
        Some(id) => {
            tx.update("services", &id, &rec).unwrap();
            let prepared = tx.prepare().unwrap();
            prepared.commit().unwrap();
            id
        }
        None => {
            let new_id = tx.insert("services", rec).unwrap();

            tx.put::<String, PersyId>("index", service_info.service.to_string(), new_id)
                .unwrap();
            let prepared = tx.prepare().unwrap();
            prepared.commit().unwrap();

            new_id
        }
    }
}

pub fn get_service(service_name: Services) -> Option<ServiceInfo> {
    let persy = get_store();

    let read_id = persy
        .get::<String, PersyId>("index", &service_name.to_string())
        .unwrap()
        .next();

    if let Some(id) = read_id {
        let value = persy.read("services", &id).unwrap();
        match value {
            Some(val) => Some(ServiceInfo::from(val)),
            None => None,
        }
    } else {
        None
    }
}

pub fn get_all_services() -> Vec<ServiceInfo> {
    let persy = get_store();
    let mut services: Vec<ServiceInfo> = vec![];
    for (read_id, content) in persy.scan("services").unwrap() {
        let service = ServiceInfo::from(content);
        services.push(service);
    }

    return services;
}
