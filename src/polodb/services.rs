use polodb_core::{Database, Error};

pub fn get_database() -> Result<Database, Error> {
    let db = Database::open_file("deleterr.db")?;
    Ok(db)
}
