use super::services::Services;
use serde::{Deserialize, Serialize};

pub enum RequestType {
    Get,
    Delete,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseCodeBasedAction {
    pub status: APIStatus,
    pub success: bool,
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
    WrongKey,
    Other,
    NotFound,
}

impl ToString for APIStatus {
    fn to_string(&self) -> String {
        match self {
            APIStatus::Success => "Sucess".to_string(),
            APIStatus::WrongKey => "Wrong API key provided".to_string(),
            APIStatus::Other => "Unknown error".to_string(),
            APIStatus::NotFound => "Record not found!".to_string(),
        }
    }
}

impl APIStatus {
    pub fn as_str(&self) -> &str {
        match self {
            APIStatus::Success => "Sucess",
            APIStatus::WrongKey => "Wrong API key provided",
            APIStatus::Other => "Unknown error",
            APIStatus::NotFound => "Record not found!",
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
