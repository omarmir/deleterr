use crate::{
    common::models::deleterr_error::DeleterrError,
    store::models::{EitherKeyType, Preferences},
};
use jammdb::{Error, DB};
use std::str;

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

pub fn save_data(bucket_name: &str, data: &[u8], key: &str) -> Result<(), Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let bucket = tx.get_or_create_bucket(bucket_name)?;
    bucket.put(key.as_bytes(), data)?;
    Ok(tx.commit()?)
}

pub fn upsert_exemption(
    bucket_name: &str,
    key: &str,
    exemption: usize,
) -> Result<(), DeleterrError> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let data_bucket = tx.get_or_create_bucket(bucket_name)?;

    let items: Vec<usize> = match data_bucket.get(key.as_bytes()) {
        Some(data) => {
            let mut exemptions =
                Preferences::to_exemptions_from_vec(Some(data.kv().value().to_vec()));
            exemptions.push(exemption);
            exemptions
        }
        None => vec![exemption],
    };

    let bytes = Preferences::to_vec_from_exemptions(&items);
    data_bucket.put(key, bytes)?;

    tx.commit()?;

    Ok(())
}

pub fn remove_exemption(
    bucket_name: &str,
    key: &str,
    exemption: usize,
) -> Result<(), DeleterrError> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let data_bucket = tx.get_or_create_bucket(bucket_name)?;

    let items: Result<Vec<usize>, DeleterrError> = match data_bucket.get(key.as_bytes()) {
        Some(data) => {
            let mut exemptions =
                Preferences::to_exemptions_from_vec(Some(data.kv().value().to_vec()));
            if let Some(index) = exemptions.iter().position(|&x| x == exemption) {
                // Remove the value at the found index
                exemptions.remove(index);
            };
            Ok(exemptions)
        }
        None => Err(DeleterrError::new("Exemption not found.")),
    };

    let bytes = Preferences::to_vec_from_exemptions(&items?);
    data_bucket.put(key, bytes)?;

    tx.commit()?;

    Ok(())
}

pub fn _remove_pair(bucket_name: &str, key_enum: EitherKeyType) -> Result<bool, Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let bucket = tx.get_bucket(bucket_name)?;

    let key: Vec<u8> = match key_enum {
        EitherKeyType::Regular(val) => val.as_bytes().to_vec(),
        EitherKeyType::Number(val) => val.to_le_bytes().to_vec(),
    };

    let deletion = match bucket.get_kv(&key).is_some() {
        true => {
            bucket.delete(&key)?;
            true
        }
        false => false,
    };

    tx.commit()?;

    Ok(deletion)
}
