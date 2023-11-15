use persy::{Config, OpenError, Persy, PersyId, ValueMode, PE};
use std::{path::Path, sync::Mutex};

use crate::common::models::DeleterrError;

use super::models::Store;

static PERSY_PATH: Mutex<Store> = 
    Mutex::new(
        Store{
            path: "deleterr.persy",
        }
    );

pub fn get_store() -> Result<Persy, PE<OpenError>> {
    let path_val = PERSY_PATH.lock().expect("Unable to obtain lock. Likely poisoned.");
    let path = Path::new(path_val.path);
    let config = Config::new();

    Persy::open_or_create_with(path, config, |persy| {
        // this closure is only called on database creation
        let mut tx = persy.begin()?;
        tx.create_segment("services")?;
        tx.create_segment("media_exemptions")?;
        tx.create_segment("user_exemption")?;
        tx.create_index::<String, PersyId>("services_index", ValueMode::Cluster)?;
        tx.create_index::<String, PersyId>("media_exemption_index", ValueMode::Cluster)?;
        tx.create_index::<String, PersyId>("user_exemption_index", ValueMode::Cluster)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        println!("Segment and Index successfully created");
        Ok(())
    })
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
