use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub success: bool,
    pub code: u16,
    pub data: APIData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum APIData {
    Success(OverseerrResponse<Request>),
    Failure(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverseerrResponse<T> {
    page_info: PageInfo,
    results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageInfo {
    page: i32,
    pages: i32,
    results: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    id: i32,
    status: i8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    requested_by: User,
    media: Media,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: i32,
    username: Option<String>,
    user_type: i32,
    email: Option<String>,
    plex_username: Option<String>,
    plex_id: Option<i64>, // Don't know if its 32 bit or 64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    id: i32,
    tmdb_id: Option<i32>,
    tvdb_id: Option<i32>,
    status: i8, // Availability of the media. 1 = UNKNOWN, 2 = PENDING, 3 = PROCESSING, 4 = PARTIALLY_AVAILABLE, 5 = AVAILABLE
}
