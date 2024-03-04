use super::common::{get_data, save_data};
use crate::{auth::models::HashedUser, common::models::deleterr_error::DeleterrError};

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
