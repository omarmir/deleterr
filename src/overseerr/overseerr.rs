use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OverseerrResponses<T> {
    List(OverseerrListResponse<T>),
    Count(OverseerrRequestsCount),
}

pub enum OverseerrResponsesTypes {
    List,
    Count,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OverseerrRequestsCount {
    total: u32,
    movie: u32,
    tv: u32,
    pending: u32,
    approved: u32,
    declined: u32,
    processing: u32,
    available: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverseerrListResponse<T> {
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
pub struct MediaRequest {
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
