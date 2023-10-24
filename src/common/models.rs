use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum APIServiceStatus {
    Success,
    WrongAPIKey,
    Other,
}

pub struct RequestResponse {
    pub code: u16,
    pub status: String,
    pub response: reqwest::Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub code: u16,
    pub data: APIData<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIData<T> {
    Success(T),
    Failure(String),
}

#[derive(Debug, Deserialize)]
pub struct DeleterrError {
    details: String,
}

impl DeleterrError {
    pub fn new(msg: &str) -> DeleterrError {
        DeleterrError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DeleterrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DeleterrError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<reqwest::Error> for DeleterrError {
    fn from(err: reqwest::Error) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}
