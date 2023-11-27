use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WatchedSeason {
    pub season_number: Option<usize>,
    pub req_status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub watched_episodes: Option<Vec<WatchedEpisode>>,
    pub watched: bool,
    pub total_items: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WatchedEpisode {
    pub external_service_id: usize,
    pub file_id: Option<usize>,
    pub watched_status: f32, //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
    pub episode_number: Option<usize>,
    pub season_number: Option<usize>,
}
