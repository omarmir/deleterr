use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct TautulliResponse {
    pub response: ResponseContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContent {
    result: Result,
    message: Option<String>,
    pub data: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub data: Option<Vec<UserWatchHistory>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Result {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserWatchHistory {
    user: String,
    friendly_name: String,
    user_id: u64, // same is plex_id on tautulli
    full_title: String,
    watched_status: f32, //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
    rating_key: u64,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    parent_rating_key: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    grandparent_rating_key: Option<u64>,
}
