use super::user_watch_history::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TautulliResponse {
    pub response: ResponseContent,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContent {
    pub result: Result,
    pub message: Option<String>,
    pub data: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub data: Option<Vec<UserWatchHistory>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Result {
    Success,
    Failure,
}
