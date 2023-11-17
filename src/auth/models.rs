use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Deserialize, Serialize)]
pub struct CookieModel {
    pub message: String,
}

pub struct User {
    pub username: String,
    pub hased_pass: String,
}

#[derive(Deserialize, Serialize, Debug, Display)]
pub enum AuthErrorType {
    Unauthorized,
    SessionStoreUnavailable,
}

#[derive(Debug, Deserialize)]
pub struct AuthError {
    pub error_type: AuthErrorType,
}

impl AuthError {
    pub fn new(error_type: AuthErrorType) -> AuthError {
        AuthError { error_type }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_type)
    }
}
