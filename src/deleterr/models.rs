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
    pub media_info: MediaInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatusWithRecordInfo {
    pub all_requests: usize,
    #[serde(with = "HashMapToArray::<usize, RequestStatus, DefaultLabels>")]
    pub requests: HashMap<usize, RequestStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatusWithRecordInfoVector {
    pub all_requests: usize,
    pub filtered_requests: usize,
    pub requests: Vec<RequestStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParms {
    pub take: Option<usize>,
    pub skip: Option<usize>,
    pub chunk: Option<usize>,
    pub sort_by: Option<SortableProps>,
    pub is_descending: Option<bool>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortableProps {
    Name,
    RequestedDate,
    MediaType,
    Watched,
    User,
}

pub struct AppData {
    pub last_update: RwLock<Option<SystemTime>>,
    pub request_cache: RwLock<Option<RequestStatusWithRecordInfo>>,
}
