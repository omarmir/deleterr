use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TautulliResponse {
    response: ResponseContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContent {
    result: Result,
    message: Option<String>,
    data: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    data: Vec<UserWatchHistory>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Result {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWatchHistory {
    user: String,
    friendly_name: String,
    user_id: u64, // same is plex_id on tautulli
    full_title: String,
    watched_status: u8, //0: Unwatched and 1: Watched
    rating_key: u64,
    parent_rating_key: Option<u64>,
    grandparent_rating_key: Option<u64>,
}
