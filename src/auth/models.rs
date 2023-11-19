extern crate actix_web;
use actix_session::Session;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};

#[derive(Deserialize, Serialize)]
pub struct CookieModel {
    pub message: String,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct AuthenticatedUser {}

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

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Session::from_request(req, pl);
        Box::pin(async move {
            let session = fut.await.expect("Unable to get sessions.");

            let session_result = session
                .get::<String>("message")
                .expect("Session store not available!");
            match session_result {
                Some(_) => return Ok(AuthenticatedUser {}),
                None => return Err(ErrorUnauthorized("Unauthorized")),
            };
        })
    }
}
