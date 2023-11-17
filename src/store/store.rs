use crate::common::models::DeleterrError;
use persy::{Persy, PersyId};

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
