use crate::{
    overseerr::models::{MediaRequest, RequestSeason},
    sonarr::series::Season,
    tautulli::user_watch_history::UserWatchHistory,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonWithStatus {
    pub season_number: Option<usize>,
    pub req_status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub watched: WatchedStatus,
    pub episode_count: usize,
    pub episode_file_count: usize,
}

impl SeasonWithStatus {
    pub fn from_series(
        season: &RequestSeason,
        watched_statuses: Option<&Vec<UserWatchHistory>>,
        series_season: Option<&Season>,
    ) -> Self {
        let watched = watched_statuses.is_watched(series_season.unwrap().statistics.episode_count);

        SeasonWithStatus {
            season_number: Some(season.season_number),
            req_status: season.status,
            watched: watched,
            episode_count: series_season.map_or(0, |season| season.statistics.episode_count),
            episode_file_count: series_season
                .map_or(0, |season| season.statistics.episode_file_count),
        }
    }

    pub fn from_movie(watched: &WatchedStatus, media_request: &MediaRequest) -> Self {
        SeasonWithStatus {
            season_number: None,
            req_status: media_request.status,
            watched: watched.clone(),
            episode_count: 1,
            episode_file_count: 1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum WatchedStatus {
    Unwatched,
    InProgress,
    Watched,
}

pub trait WatchedChecker {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus;
}

impl WatchedChecker for Option<&Vec<UserWatchHistory>> {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        match self {
            Some(histories) => {
                let watch_total = histories
                    .iter()
                    .map(|ep| ep.watched_status)
                    .fold(0.0, |acc, val| acc + val);
                watch_total.is_watched(eps_len)
            }
            None => WatchedStatus::Unwatched,
        }
    }
}

impl WatchedChecker for Vec<SeasonWithStatus> {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched_progress = self
            .iter()
            .map(|season| season.watched.clone())
            .fold(0.0, |acc, val| acc + f32::from(val))
            .is_watched(eps_len);

        watched_progress
    }
}

impl WatchedChecker for Option<UserWatchHistory> {
    fn is_watched(&self, _eps_len: usize) -> WatchedStatus {
        match self {
            Some(user_watch_history) => {
                if user_watch_history.watched_status == 0.0 {
                    WatchedStatus::Unwatched
                } else if user_watch_history.watched_status == 1.0 {
                    WatchedStatus::Watched
                } else {
                    WatchedStatus::InProgress
                }
            }
            None => WatchedStatus::Unwatched,
        }
    }
}

impl WatchedChecker for usize {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched = *self;
        if watched == 0 {
            WatchedStatus::Unwatched
        } else if watched == eps_len {
            WatchedStatus::Watched
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
            WatchedStatus::Watched
        } else {
            WatchedStatus::InProgress
        }
    }
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
