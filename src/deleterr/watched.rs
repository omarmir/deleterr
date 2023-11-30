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
    fn is_watched(&self, eps_len: usize) -> WatchedStatus;
}

impl WatchedChecker for Vec<EpisodeWithStatus> {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched_progress = self
            .iter()
            .map(|ep| ep.watched_status)
            .fold(0.0, |acc, val| acc + val);

        if watched_progress == 0.0 {
            return WatchedStatus::Unwatched;
        } else if watched_progress < eps_len as f32 {
            return WatchedStatus::InProgress;
        } else {
            // In theory watched_progress could exceed the eps_len but that would be weird.
            return WatchedStatus::Watched;
        }
    }
}

impl WatchedChecker for UserWatchHistory {
    fn is_watched(&self, _eps_len: usize) -> WatchedStatus {
        if self.watched_status == 0.0 {
            return WatchedStatus::Unwatched;
        } else if self.watched_status == 1.0 {
            return WatchedStatus::Watched;
        } else {
            return WatchedStatus::InProgress;
        }
    }
}

impl WatchedChecker for usize {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched = *self;
        if watched == 0 {
            WatchedStatus::Unwatched
        } else if watched == eps_len {
            WatchedStatus::InProgress
        } else {
            WatchedStatus::InProgress
        }
    }
}

impl WatchedChecker for f32 {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched = *self;
        if watched == 0.0 {
            WatchedStatus::Unwatched
        } else if watched == eps_len as f32 {
            WatchedStatus::InProgress
        } else {
            WatchedStatus::InProgress
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum WatchedStatus {
    Unwatched,
    InProgress,
    Watched,
}

impl From<WatchedStatus> for f32 {
    fn from(status: WatchedStatus) -> Self {
        match status {
            WatchedStatus::InProgress => 0.5,
            WatchedStatus::Unwatched => 0.0,
            WatchedStatus::Watched => 1.0,
        }
    }
}
