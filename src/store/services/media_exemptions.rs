use super::common::{get_data, remove_exemption, upsert_exemption};
use crate::{common::models::deleterr_error::DeleterrError, store::models::exemptions::Exemptions};

const BUCKET_NAME: &str = "preferences";
const KEY: &str = "media_exemptions";

pub fn get_all_exemptions() -> Result<Vec<usize>, DeleterrError> {
    let collection = get_data(BUCKET_NAME, KEY).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all exemptions.")
    })?;

    let exemptions = Exemptions::to_exemptions_from_vec(collection);

    //let exemptions = MediaExemptions::from(collection.unwrap_or(Vec::new()));

    Ok(exemptions)
}

pub fn add_media_exemption(media_exemption: usize) -> Result<(), DeleterrError> {
    let exemption_upsert = upsert_exemption(&BUCKET_NAME, KEY, media_exemption).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save media exemption.")
    });

    exemption_upsert
}

pub fn remove_media_exemption(media_exemption: usize) -> Result<(), DeleterrError> {
    let exemption_removal = remove_exemption(&BUCKET_NAME, KEY, media_exemption).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to remove media exemption.")
    });

    exemption_removal
}
