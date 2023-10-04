use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverserrRequestResponse {
    page_info: PageInfo,
    results: Vec<Request>,
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
