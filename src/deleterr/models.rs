use crate::overseerr::models::MediaRequest;
use crate::tautulli::models::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatus {
    pub media_request: MediaRequest,
    pub user_watch_history: Option<UserWatchHistory>,
}

#[derive(Deserialize)]
pub struct QueryParms {
    pub take: Option<usize>,
    pub skip: Option<usize>,
}
