use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_option_number_from_string};
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub host: String,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub port: Option<String>,
    pub api_key: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_ssl: bool,
    pub service: Services,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct APIServiceStatus {
    pub service: Services,
    pub is_success: bool,
    pub status: APIStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum APIStatus {
    Success,
    WrongAPIKey,
    Other,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Services {
    Tautulli,
    Overseerr,
}

impl From<Services> for String {
    fn from(service: Services) -> Self {
        match service {
            Services::Overseerr => "overseerr".to_string(),
            Services::Tautulli => "tautulli".to_string(),
        }
    }
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
