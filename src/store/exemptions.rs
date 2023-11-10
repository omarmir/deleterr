#![allow(dead_code)]
#![allow(unused_variables)]

use super::store::{does_record_exist, get_store};
use crate::common::models::{DeleterrError, MediaExemption};
use persy::PersyId;
use std::collections::HashMap;

pub fn upsert_media_exemption(media_exemption: MediaExemption) -> Result<String, DeleterrError> {
    let persy = get_store()?;
    let key = media_exemption.request_id.to_string();
    let value = &media_exemption.as_le_bytes();
    let persy_id = does_record_exist(&persy, &key, "media_exemption_index")?;

    let mut tx = persy.begin()?;
    match persy_id {
        Some(id) => {
            tx.update("media_exemptions", &id, value)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(id.to_string())
        }
        None => {
            let new_id = tx.insert("media_exemptions", value)?;

            tx.put::<String, PersyId>("media_exemption_index", key, new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
    }
}

pub fn get_all_exemptions() -> Result<HashMap<usize, usize>, DeleterrError> {
    let persy = get_store()?;
    let mut media_exemptions = HashMap::new();

    for (read_id, content) in persy.scan("media_exemptions")? {
        let media_exemption = MediaExemption::from(content);
        media_exemptions.insert(media_exemption.request_id, media_exemption.tmdb_id);
    }

    Ok(media_exemptions)
}
