use crate::tautulli::user_watch_history::UserWatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonWithStatus {
    pub season_number: Option<usize>,
    pub req_status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub episodes_with_status: Option<Vec<EpisodeWithStatus>>,
    pub watched: WatchedStatus,
    pub total_items: Option<usize>,
    pub last_season_with_files: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithStatus {
    pub external_service_id: usize,
    pub file_id: Option<usize>,
    pub watched_status: f32, //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
    pub episode_number: Option<usize>,
    pub season_number: Option<usize>,
}

pub trait WatchedChecker {
    fn is_watched(&self) -> WatchedStatus;
}

impl WatchedChecker for Vec<EpisodeWithStatus> {
    fn is_watched(&self) -> WatchedStatus {
        let eps_len = self.len() as f32;
        let watched_progress = self
            .iter()
            .map(|ep| ep.watched_status)
            .fold(0.0, |acc, val| acc + val);

        if watched_progress == 0.0 {
            return WatchedStatus::Unwatched;
        } else if watched_progress < eps_len {
            return WatchedStatus::InProgress;
        } else {
            // In theory watched_progress could exceed the eps_len but that would be weird.
            return WatchedStatus::Watched;
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WatchedStatus {
    Unwatched,
    InProgress,
    Watched,
}

impl WatchedChecker for UserWatchHistory {
    fn is_watched(&self) -> WatchedStatus {
        if self.watched_status == 0.0 {
            return WatchedStatus::Unwatched;
        } else if self.watched_status == 1.0 {
            return WatchedStatus::Watched;
        } else {
            return WatchedStatus::InProgress;
        }
    }
}
