use jammdb::{Error, DB};
use std::str;

const DATABASE_NAME: &str = "prunerr.db";

pub fn get_data<'a>(bucket_name: &'a str, key: &String) -> Result<Option<Vec<u8>>, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(false)?;

    let data_bucket = tx.get_bucket(bucket_name)?;
    let bucket_data = data_bucket.get(key);

    match bucket_data {
        Some(data) => {
            if data.is_kv() {
                Ok(Some(data.kv().value().to_vec()))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

pub fn get_collection(bucket_name: &str) -> Result<Vec<(String, Vec<u8>)>, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(false)?;
    let bucket = tx.get_bucket(bucket_name)?;

    //let mut results: Vec<KVPair<'b, 'tx>> = Vec::new(); // Don't know the size

    let pairs: Vec<(String, Vec<u8>)> = bucket
        .kv_pairs()
        .map(|pair| {
            (
                str::from_utf8(pair.key()).unwrap_or("").to_string(),
                pair.value().to_vec(),
            )
        })
        .collect();

    Ok(pairs)
}

pub fn get_usize_keys(bucket_name: &str) -> Result<Vec<usize>, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(false)?;
    let bucket = tx.get_bucket(bucket_name)?;

    let pairs: Vec<usize> = bucket
        .kv_pairs()
        .map(|pair| usize::from_le_bytes(pair.key().try_into().unwrap_or_default()))
        .collect();

    Ok(pairs)
}

pub fn save_data(bucket_name: &str, data: &[u8], key: &str) -> Result<(), Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let data_bucket = tx.get_bucket(bucket_name);
    match data_bucket {
        Ok(bucket) => {
            bucket.put(key, data)?;
        }
        Err(_) => {
            let data_bucket = tx.create_bucket(bucket_name)?;
            data_bucket.put(key, data)?;
        }
    }
    Ok(tx.commit()?)
}

pub fn remove_pair(bucket_name: &str, key: &str) -> Result<bool, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket(bucket_name)?;

    let deletion = match bucket.get_kv(key).is_some() {
        true => {
            bucket.delete(key)?;
            true
        }
        false => false,
    };

    Ok(deletion)
}
