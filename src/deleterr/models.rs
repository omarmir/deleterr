use crate::common::models::Services;
use crate::overseerr::models::{MediaInfo, MediaRequest};
use crate::tautulli::models::UserWatchHistory;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_option_number_from_string};

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
pub struct ServiceInfo {
    pub host: String,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub port: Option<u16>,
    pub api_key: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_ssl: bool,
    pub service: Services,
}
