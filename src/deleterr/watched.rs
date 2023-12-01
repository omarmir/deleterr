use std::collections::HashMap;

use crate::{
    overseerr::models::MediaRequest,
    sonarr::models::{EpisodeSeason, SonarrShow},
    tautulli::user_watch_history::UserWatchHistory,
};
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

impl SeasonWithStatus {
    pub fn from_show(season_eps: EpisodesWithStatus, show: &SonarrShow, status: u8) -> Self {
        SeasonWithStatus {
            season_number: Some(season_eps.season_number),
            req_status: status,
            watched: season_eps.eps.is_watched(season_eps.episode_count),
            episodes_with_status: Some(season_eps.eps),
            total_items: Some(season_eps.episode_count),
            last_season_with_files: match show.max_season_with_file {
                Some(max_season) => max_season == season_eps.season_number,
                None => false, // If there is no max_season then maybe seasons aren't in DB - be safe its not last.
            },
        }
    }

    pub fn from_movie(watched: &WatchedStatus, media_request: &MediaRequest) -> Self {
        SeasonWithStatus {
            season_number: None,
            req_status: media_request.status,
            episodes_with_status: Some(EpisodesWithStatus::get_eps_for_movie(
                media_request.media.external_service_id,
                watched,
            )),
            watched: watched.clone(),
            total_items: Some(1),
            last_season_with_files: true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeWithStatus {
    external_service_id: Option<usize>,
    file_id: Option<usize>,
    watched_status: f32, //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
    episode_number: Option<usize>,
    season_number: Option<usize>,
}

pub struct EpisodesWithStatus {
    pub eps: Vec<EpisodeWithStatus>,
    pub season_number: usize,
    pub episode_count: usize,
}

impl EpisodesWithStatus {
    pub fn get_eps_for_movie(
        external_service_id: Option<usize>,
        watched: &WatchedStatus,
    ) -> Vec<EpisodeWithStatus> {
        vec![EpisodeWithStatus {
            external_service_id: external_service_id,
            file_id: None,
            watched_status: f32::from(watched.clone()),
            episode_number: None,
            season_number: None,
        }]
    }

    pub fn from(
        episodes: &EpisodeSeason,
        tau_hist: &HashMap<(usize, usize), UserWatchHistory>,
    ) -> Self {
        let mut episodes_with_status: Vec<EpisodeWithStatus> =
            Vec::with_capacity(episodes.episode_count);
        for episode in &episodes.episodes {
            let watched_status = tau_hist
                .get(&(episode.1.season_number, episode.1.episode_number))
                .map_or(0.0, |hist| hist.watched_status);
            let watched_episode = EpisodeWithStatus {
                external_service_id: Some(episode.1.id),
                file_id: Some(episode.1.episode_file_id),
                watched_status,
                episode_number: Some(episode.0.to_owned()),
                season_number: Some(episode.1.season_number),
            };
            episodes_with_status.push(watched_episode);
        }
        EpisodesWithStatus {
            eps: episodes_with_status,
            season_number: episodes.season_number,
            episode_count: episodes.episode_count,
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

impl WatchedChecker for Vec<EpisodeWithStatus> {
    fn is_watched(&self, eps_len: usize) -> WatchedStatus {
        let watched_progress = self
            .iter()
            .map(|ep| ep.watched_status)
            .fold(0.0, |acc, val| acc + val);

        if watched_progress == 0.0 {
            return WatchedStatus::Unwatched;
        } else if watched_progress == eps_len as f32 {
            return WatchedStatus::Watched;
        } else {
            // In theory watched_progress could exceed the eps_len but that would be weird.
            return WatchedStatus::InProgress;
        }
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
