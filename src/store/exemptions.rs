#![allow(dead_code)]
#![allow(unused_variables)]

use super::store::{does_record_exist, get_store};
use crate::common::models::DeleterrError;
use persy::PersyId;
use std::collections::HashMap;

pub fn upsert_media_exemption(id: usize) -> Result<String, DeleterrError> {
    let persy = get_store()?;
    let persy_id = does_record_exist(&persy, &id, "media_exemption_index")?;

    let mut tx = persy.begin()?;
    let rec = &id.to_le_bytes();
    match persy_id {
        Some(id) => {
            tx.update("media_exemptions", &id, rec)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(id.to_string())
        }
        None => {
            let new_id = tx.insert("media_exemptions", rec)?;

            tx.put::<String, PersyId>("media_exemption_index", id.to_string(), new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
    }
}

pub fn get_all_exemptions() -> Result<HashMap<usize, usize>, DeleterrError> {
    let persy = get_store()?;
    let mut media_exemptions = HashMap::new();

    for (_read_id, content) in persy.scan("media_exemptions")? {
        let media_id = usize::from_le_bytes(
            content
                .as_slice()
                .try_into()
                .expect("Unable to read id from store for media exemption."),
        );
        media_exemptions.insert(media_id, media_id);
    }

    Ok(media_exemptions)
}
