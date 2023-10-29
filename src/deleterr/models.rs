use crate::overseerr::models::{MediaInfo, MediaRequest};
use crate::tautulli::models::UserWatchHistory;
use serde::{Deserialize, Serialize};

use serde_map_to_array::{DefaultLabels, HashMapToArray};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatus {
    pub media_request: MediaRequest,
    pub user_watch_history: Option<UserWatchHistory>,
    pub media_info: Option<MediaInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatusWithRecordInfo {
    pub all_requests: usize,
    #[serde(with = "HashMapToArray::<usize, RequestStatus, DefaultLabels>")]
    pub requests: HashMap<usize, RequestStatus>,
}

#[derive(Deserialize)]
pub struct QueryParms {
    pub take: Option<usize>,
}

pub struct AppData {
    pub last_update: RwLock<Option<SystemTime>>,
    pub request_cache: RwLock<Option<RequestStatusWithRecordInfo>>,
}
