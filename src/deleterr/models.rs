use super::watched::{SeasonWithStatus, WatchedStatus};
use crate::radarr::models::Movie;
use crate::sonarr::series::Series;
use crate::sonrad::models::Image;
use crate::{common::models::ResponseCodeBasedAction, overseerr::models::MediaRequest};
use serde::{Deserialize, Serialize};
use serde_map_to_array::{DefaultLabels, HashMapToArray};
use std::cmp;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatus {
    pub media_request: MediaRequest,
    pub season_status: Vec<SeasonWithStatus>,
    pub watched: WatchedStatus,
    pub media_info: MediaInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub images: Option<Vec<Image>>,
    pub release_date: Option<String>,
    pub title: String,
    pub ended: Option<bool>,
}

impl MediaInfo {
    pub fn default() -> Self {
        MediaInfo {
            images: None,
            release_date: None,
            title: "N/A".to_string(),
            ended: None,
        }
    }

    pub fn new(
        release_date: Option<String>,
        title: String,
        images: Option<Vec<Image>>,
        ended: Option<bool>,
    ) -> Self {
        MediaInfo {
            images,
            release_date,
            title,
            ended,
        }
    }

    pub fn from_sonarr(series: Option<Series>) -> Self {
        match series {
            Some(series) => MediaInfo::new(
                series.first_aired,
                series.title,
                series.images,
                Some(series.ended),
            ),
            None => MediaInfo::default(),
        }
    }

    pub fn from_radarr(movie: Option<Movie>) -> Self {
        match movie {
            Some(movie) => {
                let release_date = match (movie.digital_release, movie.physical_release) {
                    (Some(digital_release), Some(physical_release)) => {
                        Some(cmp::min(digital_release, physical_release))
                    }
                    (None, Some(physical_release)) => Some(physical_release),
                    (Some(digital_release), None) => Some(digital_release),
                    (None, None) => None,
                };

                MediaInfo::new(release_date, movie.title, movie.images, None)
            }
            None => MediaInfo::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatusWithRecordInfo {
    pub all_requests: usize,
    #[serde(with = "HashMapToArray::<usize, RequestStatus, DefaultLabels>")]
    pub requests: HashMap<usize, RequestStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestStatusWithRecordInfoVector {
    pub all_requests: usize,
    pub filtered_requests: usize,
    pub requests: Vec<RequestStatus>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieDeletionRequest {
    pub radarr_response: ResponseCodeBasedAction,
    pub overseerr_response: ResponseCodeBasedAction,
    pub cache_response: ResponseCodeBasedAction,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParms {
    pub take: Option<usize>,
    pub skip: Option<usize>,
    pub chunk: Option<usize>,
    pub sort_by: Option<SortableProps>,
    pub is_descending: Option<bool>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortableProps {
    Name,
    RequestedDate,
    MediaType,
    Watched,
    User,
}

pub struct AppData {
    pub last_update: RwLock<Option<SystemTime>>,
    pub request_cache: RwLock<Option<RequestStatusWithRecordInfo>>,
}
