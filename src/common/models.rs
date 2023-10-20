use enum_as_inner::EnumAsInner;
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

#[derive(Debug, Serialize, Deserialize, EnumAsInner)]
#[serde(untagged)]
pub enum APIData<T> {
    Success(T),
    Failure(String),
}
