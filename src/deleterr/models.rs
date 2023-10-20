use crate::overseerr::models::MediaRequest;
use crate::tautulli::models::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestStatus {
    pub media_request: MediaRequest,
    pub user_watch_history: Option<UserWatchHistory>,
}