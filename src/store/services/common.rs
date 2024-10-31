use crate::{common::models::deleterr_error::DeleterrError, store::models::exemptions::Exemptions};
use jammdb::{Error, DB};
use std::str;

const DATABASE_NAME: &str = "prunerr.db";

/// Retrieves data from a database bucket by key, creating the bucket if it doesn't
/// exist.
///
/// # Arguments:
///
/// * `bucket_name`: The name of the bucket from which data needs to be retrieved.
/// * `key`: The `key` used to retrieve data from a specific bucket in the database.
///
/// # Returns:
///
/// A `Result` containing an `Option` of a `Vec<u8>` or an `Error`.
/// The data would need to be reconstructed from the Vec<u8> to be used.
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

/// Retrieves a collection of key-value pairs from a database bucket, creating the
/// bucket if it doesn't exist.
///
/// # Arguments:
///
/// * `bucket_name`: The name of the bucket from which collection needs to be retrieved.
///
/// # Returns:
///
/// A `Result` containing a vector of tuples. Each tuple consists of a `String` and a `Vec<u8>`.
/// The `String` represents the key, while the `Vec<u8>` represents the
/// value stored in the collection bucket specified by the `bucket_name` parameter.
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

/// Saves data to a specified bucket in a database.
///
/// # Arguments:
///
/// * `bucket_name`: Name of the bucket where the data will be saved.
/// * `data`: The actual data that you want to save in the database, expects a slice of bytes (`&[u8]`)
/// * `key`: The key under which the data will be saved in the specified bucket.
///
/// # Returns:
///
/// A `Result<(), Error>` indicating successful (or errnoeous) operation.
pub fn save_data(bucket_name: &str, data: &[u8], key: &str) -> Result<(), Error> {
    let db = DB::open(DATABASE_NAME)?;
    let tx = db.tx(true)?;

    let bucket = tx.get_or_create_bucket(bucket_name)?;
    bucket.put(key.as_bytes(), data)?;
    Ok(tx.commit()?)
}

/// Upserts (update if exist/create new if it doesn't) an exemption value into the database bucket.
///
/// # Arguments:
///
/// * `bucket_name`: Name of the bucket where the data will be saved.
/// * `key`: The key under which the data will be saved in the specified bucket.
/// * `exemption`: A `usize` ID of the Overseerr request
///
/// # Returns:
///
/// A `Result<(), Error>` indicating successful (or errnoeous) operation.
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
                Exemptions::to_exemptions_from_vec(Some(data.kv().value().to_vec()));
            exemptions.push(exemption);
            exemptions
        }
        None => vec![exemption],
    };

    let bytes = Exemptions::to_vec_from_exemptions(&items);
    data_bucket.put(key, bytes)?;

    tx.commit()?;

    Ok(())
}

/// Removes a specific exemption from a data bucket in the database.
///
/// # Arguments:
///
/// * `bucket_name`: Name of the bucket where the data will be saved.
/// * `key`: The key under which the data will be saved in the specified bucket.
/// * `exemption`: A `usize` ID of the Overseerr request
///
/// # Returns:
///
/// A `Result<(), Error>` indicating successful (or errnoeous) operation.
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
                Exemptions::to_exemptions_from_vec(Some(data.kv().value().to_vec()));
            if let Some(index) = exemptions.iter().position(|&x| x == exemption) {
                // Remove the value at the found index
                exemptions.remove(index);
            };
            Ok(exemptions)
        }
        None => Err(DeleterrError::new("Exemption not found.")),
    };

    let bytes = Exemptions::to_vec_from_exemptions(&items?);
    data_bucket.put(key, bytes)?;

    tx.commit()?;

    Ok(())
}
