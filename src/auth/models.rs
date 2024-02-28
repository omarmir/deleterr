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
}

impl From<Vec<u8>> for HashedUser {
    fn from(bytes: Vec<u8>) -> Self {
        let hashed_user: HashedUser =
            serde_json::from_slice(&bytes).expect("Failed to deserialize user info");

        hashed_user
    }
}
