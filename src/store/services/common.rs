use jammdb::{Error, DB};
use std::str;

use crate::store::models::EitherKeyType;

const DATABASE_NAME: &str = "prunerr.db";

pub fn get_data<'a>(bucket_name: &'a str, key: &str) -> Result<Option<Vec<u8>>, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let mut tx = db.tx(false)?;

    let data_bucket = match tx.get_bucket(bucket_name) {
        Ok(bucket) => bucket,
        Err(_) => {
            tx = db.tx(true)?;
            let bucket = tx.create_bucket(bucket_name)?;
            bucket
        }
    };

    let bucket_data = data_bucket.get(key.as_bytes());

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
    let mut tx = db.tx(false)?;

    let data_bucket = match tx.get_bucket(bucket_name) {
        Ok(bucket) => bucket,
        Err(_) => {
            tx = db.tx(true)?;
            let bucket = tx.create_bucket(bucket_name)?;
            bucket
        }
    };

    let pairs: Vec<(String, Vec<u8>)> = data_bucket
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
    let mut tx = db.tx(false)?;

    let data_bucket = match tx.get_bucket(bucket_name) {
        Ok(bucket) => bucket,
        Err(_) => {
            tx = db.tx(true)?;
            let bucket = tx.create_bucket(bucket_name)?;
            bucket
        }
    };

    let pairs: Vec<usize> = data_bucket
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
            bucket.put(key.as_bytes(), data)?;
        }
        Err(_) => {
            let data_bucket = tx.create_bucket(bucket_name)?;
            data_bucket.put(key, data)?;
        }
    }
    Ok(tx.commit()?)
}

pub fn save_usize_keys_only(bucket_name: &str, key: &usize) -> Result<(), Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;
    let empty_bytes: &[u8] = &[];

    let data_bucket = tx.get_bucket(bucket_name);
    match data_bucket {
        Ok(bucket) => {
            bucket.put(key.to_le_bytes(), empty_bytes)?;
        }
        Err(_) => {
            let data_bucket = tx.create_bucket(bucket_name)?;
            data_bucket.put(key.to_le_bytes(), empty_bytes)?;
        }
    }
    Ok(tx.commit()?)
}

pub fn remove_pair(bucket_name: &str, key_enum: EitherKeyType) -> Result<bool, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;
    let data_bucket = match tx.get_bucket(bucket_name) {
        Ok(bucket) => bucket,
        Err(_) => {
            let bucket = tx.create_bucket(bucket_name)?;
            bucket
        }
    };

    let key: Vec<u8> = match key_enum {
        EitherKeyType::Regular(val) => val.as_bytes().to_vec(),
        EitherKeyType::Number(val) => val.to_le_bytes().to_vec(),
    };

    let deletion = match data_bucket.get_kv(&key).is_some() {
        true => {
            data_bucket.delete(&key)?;
            true
        }
        false => false,
    };

    tx.commit()?;

    Ok(deletion)
}
