use super::common::{get_collection, get_data, save_data};
use crate::{
    auth::models::{HashedUser, User},
    common::models::deleterr_error::DeleterrError,
};

const BUCKET_NAME: &str = "users";

pub fn get_user_by_username(username: &String) -> Result<HashedUser, DeleterrError> {
    let users_data = get_data(BUCKET_NAME, &username).unwrap_or(None);

    match users_data {
        Some(data) => {
            let hashed_user: HashedUser = HashedUser::from_vec(data)?;
            Ok(hashed_user)
        }
        None => Err(DeleterrError::new("User not found")),
    }
}

pub fn add_user_to_store(user: HashedUser) -> Result<(), DeleterrError> {
    let existing_user = get_user_by_username(&user.username);

    if existing_user.is_ok() {
        return Err(DeleterrError::new("User already exists"));
    }

    let user_add = save_data(&BUCKET_NAME, &user.as_vec(), &user.username);

    match user_add {
        Ok(_) => Ok(()),
        Err(err) => Err(DeleterrError::from(err).add_prefix("Unable to add user.")),
    }
}

pub fn update_user_in_store(user: HashedUser) -> Result<(), DeleterrError> {
    let existing_user = get_user_by_username(&user.username);

    if existing_user.is_err() {
        return Err(DeleterrError::new("User not found"));
    }

    let user_update = save_data(&BUCKET_NAME, &user.as_vec(), &user.username);

    match user_update {
        Ok(_) => Ok(()),
        Err(err) => Err(DeleterrError::from(err).add_prefix("Unable to update user.")),
    }
}

pub fn is_users_setup() -> Result<bool, DeleterrError> {
    let user_collection = get_collection(&BUCKET_NAME)?;

    match user_collection.len() {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub fn add_user(unhashed_user: User) -> Result<(), DeleterrError> {
    let hashed_user = HashedUser::from_user(unhashed_user)?;
    add_user_to_store(hashed_user)
}

pub fn initialize_user(unhashed_user: User) -> Result<(), DeleterrError> {
    let is_users_setup = is_users_setup()?;
    match is_users_setup {
        true => Err(DeleterrError::new(
            "Cannot initialize users when there is already a user setup.",
        )),
        false => {
            let hashed_user = HashedUser::from_user(unhashed_user)?;
            add_user_to_store(hashed_user)
        }
    }
}
