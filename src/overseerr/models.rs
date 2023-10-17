use super::overseerr::OverseerrResponses;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub code: u16,
    pub data: APIData<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum APIData<T> {
    Success(OverseerrResponses<T>),
    Failure(String),
}
