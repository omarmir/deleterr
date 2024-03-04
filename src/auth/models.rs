use crate::common::models::deleterr_error::DeleterrError;
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct CookieModel {
    pub message: String,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct HashedUser {
    pub username: String,
    pub hash: String,
}

impl HashedUser {
    pub fn as_vec(&self) -> Vec<u8> {
        let user_bytes = serde_json::to_vec(&self).expect("Failed to serialize user to JSON");
        user_bytes
    }

    pub fn from_user(user: User) -> Result<Self, DeleterrError> {
        let hashed_pass = Self::hash_password(user.password)?;
        Ok(Self {
            username: user.username,
            hash: hashed_pass,
        })
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, DeleterrError> {
        let hashed_user: HashedUser = serde_json::from_slice(&bytes).map_err(|e| {
            DeleterrError::new(e.to_string().as_str()).add_prefix("Unable to deserialize user.")
        })?;

        Ok(hashed_user)
    }

    fn hash_password(pass: String) -> Result<String, DeleterrError> {
        let hash = hash(pass, 10)
            .map_err(|e| {
                DeleterrError::new(e.to_string().as_str()).add_prefix("Unable to hash password.")
            })?
            .to_string();

        Ok(hash)
    }

    pub fn verify_hash(&self, pass: String) -> Result<bool, DeleterrError> {
        let matches = verify(pass, self.hash.as_str()).map_err(|e| {
            DeleterrError::new(e.to_string().as_str()).add_prefix("Unable to verify password hash.")
        })?;

        Ok(matches)
    }
}
