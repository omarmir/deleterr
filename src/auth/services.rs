use super::models::HashedUser;
use crate::{
    auth::models::User,
    common::models::DeleterrError,
    store::store::{does_record_exist, get_persy},
};
use actix_session::Session;
use bcrypt::{hash, verify};
use persy::{Persy, PersyId};

pub fn validate_session(session: Session) -> bool {
    let result = session.get::<String>("message");
    let resp = result.expect("Session store not available.");
    match resp {
        Some(_) => true,
        None => false,
    }
}

pub fn login_user(session: Session, user: User) -> Result<String, DeleterrError> {
    let username = user.username.clone();
    let is_login_success = verify_user(user)?;
    if is_login_success {
        session
            .insert("message", username.clone())
            .map_err(|err| DeleterrError::new(err.to_string().as_str()))?;
        Ok(username)
    } else {
        Err(DeleterrError::new(
            "Login failed. Username or password incorrect.",
        ))
    }
}

pub fn verify_user(unhashed_user: User) -> Result<bool, DeleterrError> {
    let persy = get_persy()?;

    let user = get_user_by_username(unhashed_user.username, &persy)?;

    match user {
        Some(user) => {
            let matches = verify(unhashed_user.password, user.hash.as_str()).map_err(|e| {
                DeleterrError::new(e.to_string().as_str())
                    .add_prefix(" Unable to verify password hash.")
            })?;
            Ok(matches)
        }
        None => Ok(false),
    }
}

pub fn get_user_by_username(
    username: String,
    persy: &Persy,
) -> Result<Option<HashedUser>, DeleterrError> {
    let read_id = persy
        .get::<String, PersyId>("users_index", &username)?
        .next();

    if let Some(id) = read_id {
        let value = persy.read("users", &id)?;
        match value {
            Some(val) => Ok(Some(HashedUser::from(val))),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn hash_password(pass: String) -> Result<String, DeleterrError> {
    let hash = hash(pass, 10)
        .map_err(|e| {
            DeleterrError::new(e.to_string().as_str()).add_prefix(" Unable to hash password.")
        })?
        .to_string();

    Ok(hash)
}

pub fn upsert_user(unhashed_user: User) -> Result<String, DeleterrError> {
    let hash = hash_password(unhashed_user.password)?;

    let user = HashedUser {
        username: unhashed_user.username,
        hash,
    };

    let persy = get_persy()?;
    //Start a transaction all the operations in persy are done inside a transaction.
    let persy_id = does_record_exist(&persy, &user.username, "users_index")?;

    let mut tx = persy.begin()?;
    let rec = &user.as_bytes();
    match persy_id {
        Some(id) => {
            tx.update("users", &id, &rec)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(id.to_string())
        }
        None => {
            let new_id = tx.insert("users", &rec)?;

            tx.put::<String, PersyId>("users_index", user.username.to_string(), new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
    }
}
