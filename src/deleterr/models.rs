use crate::overseerr::models::{MediaInfo, MediaRequest};
use crate::tautulli::models::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatus {
    pub media_request: MediaRequest,
    pub user_watch_history: Option<UserWatchHistory>,
    pub media_info: Option<MediaInfo>,
}

#[derive(Deserialize)]
pub struct QueryParms {
    pub take: Option<usize>,
    pub skip: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub host: String,
    pub port: Option<u8>,
    pub api_key: String,
    pub use_ssl: bool,
    pub service: Services,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Services {
    Tautulli,
    Overseer,
}
