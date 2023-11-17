use super::store::does_record_exist;
use crate::{
    common::models::{DeleterrError, MediaExemption},
    PERSY_MANAGER,
};
use persy::PersyId;
use std::collections::HashMap;

// TODO: We need to update the cache to reflect the media exemption or we should send the data as seperate and hashmap and the front end can display it as needed
pub fn upsert_media_exemption(media_exemption: MediaExemption) -> Result<String, DeleterrError> {
    let persy = PERSY_MANAGER
        .persy
        .lock()
        .expect("Unable to obtain lock on store. Likely poisoned. Restart app");

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
    let persy = PERSY_MANAGER
        .persy
        .lock()
        .expect("Unable to obtain lock on store. Likely poisoned. Restart app");

    let mut media_exemptions = HashMap::new();

    for (read_id, content) in persy.scan("media_exemptions")? {
        let media_exemption = MediaExemption::from(content);
        media_exemptions.insert(media_exemption.request_id, media_exemption.tmdb_id);
    }

    Ok(media_exemptions)
}

pub fn remove_media_exemption(request_id: usize) -> Result<bool, DeleterrError> {
    let persy = PERSY_MANAGER
        .persy
        .lock()
        .expect("Unable to obtain lock on store. Likely poisoned. Restart app");

    let key = request_id.to_string();
    let persy_id = does_record_exist(&persy, &key, "media_exemption_index")?;
    let mut tx = persy.begin()?;
    match persy_id {
        Some(id) => {
            tx.delete("media_exemptions", &id)?;
            tx.remove::<String, PersyId>("media_exemption_index", key, None)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(true)
        }
        None => Ok(false),
    }
}
