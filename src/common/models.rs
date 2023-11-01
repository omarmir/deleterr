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

impl ServiceInfo {
    pub fn as_bytes(&self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self).expect("Failed to serialize to JSON");
        let bytes = json_str.as_bytes();
        bytes.to_vec()
    }
}

impl From<Vec<u8>> for ServiceInfo {
    fn from(bytes: Vec<u8>) -> Self {
        // Convert the bytes to a JSON string
        let json_str = String::from_utf8_lossy(&bytes).to_string();

        // Deserialize the JSON string into your struct
        let service_info: ServiceInfo =
            serde_json::from_str(&json_str).expect("Failed to deserialize from JSON");

        service_info
    }
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

impl Services {
    pub fn to_string(&self) -> String {
        match self {
            Services::Tautulli => "tautulli".to_string(),
            Services::Overseerr => "overseerr".to_string(),
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
    pub data: Option<T>,
    pub error_msg: Option<String>,
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
