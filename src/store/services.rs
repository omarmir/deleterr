#![allow(dead_code)]
#![allow(unused_variables)]

use persy::{Config, Persy, PersyId};

use crate::common::models::ServiceInfo;

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
    //Prepere some raw data
    let encoded: Vec<u8> = bincode::serialize(&service_info).unwrap();
    //Insert the data inside the segment with the current tx.
    let id = tx
        .insert(service_info.service.as_str(), encoded.as_slice())
        .unwrap();
    //Commit the tx.
    let prepared = tx.prepare().unwrap();
    prepared.commit().unwrap();

    return id;
}
