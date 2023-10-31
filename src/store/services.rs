#![allow(dead_code)]
#![allow(unused_variables)]

use persy::{Config, Persy, PersyId};

use crate::common::models::{ServiceInfo, Services};

pub fn create_store() {
    //Create the file
    Persy::create("./deleterr.persy").unwrap();
}

pub fn save_service(service_info: ServiceInfo) -> PersyId {
    let persy = Persy::open("./deleterr.persy", Config::new()).unwrap();
    //Start a transaction all the operations in persy are done inside a transaction.
    let mut tx = persy.begin().unwrap();
    //Create a segment called "seg" using the started tx.
    tx.create_segment(service_info.service.as_str()).unwrap();
    let id = tx
        .insert(service_info.service.as_str(), &service_info.as_bytes())
        .unwrap();
    //Commit the tx.
    let prepared = tx.prepare().unwrap();
    prepared.commit().unwrap();

    return id;
}

pub fn get_service(service_name: Services) -> Option<ServiceInfo> {
    let persy = Persy::open("./deleterr.persy", Config::new()).unwrap();
    for (read_id, content) in persy.scan(service_name.as_str()).unwrap() {
        //.... do something with the record.id and record.content
        //let service = bincode::deserialize(&content).unwrap();
        let service = ServiceInfo::from(content);
        return Some(service);
    }
    None
}
