use jammdb::{Error, DB};
const DATABASE_NAME: &str = "prunerr.db";

pub fn get_data<'a>(bucket_name: &str, key: String) -> Result<Option<&'a [u8]>, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let mut tx = db.tx(false)?;

    let data_bucket = tx.get_bucket(bucket_name)?;
    let bucket_data = data_bucket.get(key);

    match bucket_data {
        Some(data) => {
            if data.is_kv() {
                Ok(Some(data.kv().value()))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

pub fn save_data(bucket_name: &str, data: &[u8], key: &str) -> Result<(), Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let data_bucket = tx.get_bucket(bucket_name);
    match data_bucket {
        Ok(bucket) => {
            let data_bucket = tx.get_bucket(bucket_name)?;
            data_bucket.put(key, data)?;
        }
        Err(_) => {
            let data_bucket = tx.create_bucket(bucket_name)?;
            data_bucket.put(key, data)?;
        }
    }
    Ok(tx.commit()?)
}
