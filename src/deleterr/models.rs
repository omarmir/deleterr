use crate::overseerr::models::OverseerrResponses;
use serde::{Deserialize, Serialize};

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
    OverseerrSuccess(OverseerrResponses<T>),
    RadarrSuccess,
    SonarrSuccess,
    TautulliSuccess,
    Failure(String),
}
