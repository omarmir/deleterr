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
    pub fn as_bytes(&self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self).expect("Failed to serialize to JSON");
        let bytes = json_str.as_bytes();
        bytes.to_vec()
    }
}

impl From<Vec<u8>> for HashedUser {
    fn from(bytes: Vec<u8>) -> Self {
        // Convert the bytes to a JSON string
        let json_str = String::from_utf8_lossy(&bytes).to_string();

        // Deserialize the JSON string into your struct
        let user: HashedUser =
            serde_json::from_str(&json_str).expect("Failed to deserialize from JSON");

        user
    }
}
