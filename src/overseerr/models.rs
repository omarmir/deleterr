use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct OverseerrRequestsCount {
    total: u32,
    movie: u32,
    tv: u32,
    pending: u32,
    approved: u32,
    declined: u32,
    processing: u32,
    pub available: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverseerrListResponse<T> {
    pub page_info: PageInfo,
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageInfo {
    pub page: u32,
    pub pages: usize,
    pub results: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaRequest {
    pub id: u32,
    pub status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub requested_by: User,
    pub media: Media,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    username: Option<String>,
    user_type: u32,
    email: Option<String>,
    plex_username: Option<String>,
    pub plex_id: Option<u64>, // Don't know if its 32 bit or 64 - same as tautulli user
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    id: u32,
    media_type: Option<MediaType>,
    tmdb_id: Option<u32>,
    tvdb_id: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub rating_key: Option<u64>,
    status: u8, // Availability of the media. 1 = UNKNOWN, 2 = PENDING, 3 = PROCESSING, 4 = PARTIALLY_AVAILABLE, 5 = AVAILABLE
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum MediaType {
    Movie,
    TV,
}
