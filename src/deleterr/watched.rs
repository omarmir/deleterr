use crate::tautulli::user_watch_history::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Watched {
    pub req_parent_index_id: usize,
    pub parent_media_index: Option<usize>,
    pub req_status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub watch_history: Vec<Option<UserWatchHistory>>,
    pub watched: bool,
    pub total_items: Option<usize>,
}
