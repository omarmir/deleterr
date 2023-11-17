use std::sync::MutexGuard;

use crate::{common::models::DeleterrError, PERSY_MANAGER};
use persy::{Persy, PersyId};

pub fn get_persy() -> Result<MutexGuard<'static, Persy>, DeleterrError> {
    let persy = PERSY_MANAGER
        .get()
        .ok_or(DeleterrError::new("Missing media Id!"))?
        .persy
        .lock()
        .map_err(|err| {
            DeleterrError::new(err.to_string().as_str())
                .add_prefix("Unable to access cache. Lock is poisoned.")
        })?;

    Ok(persy)
}

pub fn does_record_exist<T>(
    persy: &Persy,
    k: &T,
    index_name: &str,
) -> Result<Option<PersyId>, DeleterrError>
where
    T: ToString,
{
    let read_id = persy
        .get::<String, PersyId>(index_name, &k.to_string())?
        .next();

    if let Some(id) = read_id {
        Ok(Some(id))
    } else {
        Ok(None)
    }
}
