use super::common::{get_data, save_data};
use crate::{common::models::deleterr_error::DeleterrError, store::models::settings::Settings};

const BUCKET_NAME: &str = "preferences";
const KEY: &str = "settings";

pub fn get_all_settings() -> Result<Settings, DeleterrError> {
    let data = get_data(BUCKET_NAME, KEY).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to get all settings.")
    })?;

    if let Some(sett) = data {
        let settings: Settings = Settings::from(sett);
        Ok(settings)
    } else {
        Ok(Settings::default())
    }
}

pub fn save_settings(settings: Settings) -> Result<(), DeleterrError> {
    let save_setting = save_data(&BUCKET_NAME, &settings.as_vec(), KEY).map_err(|err| {
        DeleterrError::new(err.to_string().as_str()).add_prefix("Unable to save settings.")
    });

    save_setting
}
