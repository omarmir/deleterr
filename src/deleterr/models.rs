use super::watched::{SeasonWithStatus, WatchedStatus};
use crate::common::broadcast::Broadcaster;
use crate::common::models::api::{APIStatus, ResponseCodeBasedAction};
use crate::common::models::deleterr_error::DeleterrError;
use crate::overseerr::models::MediaRequest;
use crate::radarr::models::Movie;
use crate::sonarr::series::Series;
use crate::sonrad::models::Image;
use serde::{Deserialize, Serialize};
use serde_map_to_array::{DefaultLabels, HashMapToArray};
use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDeletionRequest {
    pub request_fully_watched: bool,
    pub sonarr_response: ResponseCodeBasedAction,
    pub cache_response: ResponseCodeBasedAction,
    pub overseerr_response: Option<ResponseCodeBasedAction>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDeletionEpisodes {
    pub episodes: Vec<usize>,
    pub request_fully_watched: bool,
}

impl SeriesDeletionEpisodes {
    pub fn default() -> Self {
        SeriesDeletionEpisodes {
            episodes: Vec::with_capacity(0),
            request_fully_watched: false,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParms {
    pub take: Option<usize>,
    pub skip: Option<usize>,
    pub sort_by: Option<SortableProps>,
    pub is_descending: Option<bool>,
    pub search: Option<String>,
}

impl QueryParms {
    pub fn default() -> Self {
        QueryParms {
            take: None,
            skip: None,
            sort_by: None,
            is_descending: None,
            search: None,
        }
    }
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

#[derive(Clone, Copy)]
pub enum CacheStatus {
    Uninitialized,
    Building,
    Built,
}

// Last update is unimplemented thus far
#[allow(dead_code)]
pub struct AppData {
    pub last_update: RwLock<Option<SystemTime>>,
    pub request_cache: RwLock<Option<RequestStatusWithRecordInfo>>,
    pub broadcaster: Arc<Broadcaster>,
    pub cache_status: RwLock<CacheStatus>,
}

impl AppData {
    pub fn default() -> Self {
        return AppData {
            last_update: RwLock::new(None),
            request_cache: RwLock::new(None),
            broadcaster: Broadcaster::create(),
            cache_status: RwLock::new(CacheStatus::Uninitialized),
        };
    }

    pub fn get_request(&self, req_id: &usize) -> Option<RequestStatus> {
        let cache_lock = self
            .request_cache
            .read()
            .expect("Unable to access cache")
            .clone();
        match cache_lock {
            Some(reqs) => reqs.requests.get(&req_id).cloned(),
            None => None,
        }
    }

    pub fn delete_request(&self, req_id: &usize) -> Result<ResponseCodeBasedAction, DeleterrError> {
        let mut update_cache = self
            .request_cache
            .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
            .map_err(|err| {
                DeleterrError::new(err.to_string().as_str())
                    .add_prefix("Unable to access cache. Lock is poisoned.")
            })?;

        if let Some(del_cache) = update_cache.as_mut() {
            del_cache.requests.remove(req_id);
        } else {
            return Err(DeleterrError::new(
                "Cache is empty. Maybe resync the cache first?",
            ));
        }
        Ok(ResponseCodeBasedAction {
            status: APIStatus::Success,
            success: true,
        })
    }

    pub fn set_cache_is_building(&self) {
        let mut update_cache = self
            .cache_status
            .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
            .expect("Unable to access cache");

        *update_cache = CacheStatus::Building
    }

    pub fn set_cache_is_built(&self) {
        let mut update_cache = self
            .cache_status
            .write() // ! This could leave the app timed out waiting for a write lock - I can't think when/why this would happen
            .expect("Unable to access cache");

        *update_cache = CacheStatus::Built
    }

    pub fn cache_status(&self) -> CacheStatus {
        let cache = self.cache_status.read().expect("Unable to access cache");
        return *cache;
    }

    pub fn clear_cache(&self) -> Result<(), DeleterrError> {
        let cache_status = self.cache_status.write();
        let cache = self.request_cache.write();

        let cache_clear = match (cache_status, cache) {
            (Ok(mut status), Ok(mut reqs)) => {
                *reqs = None;
                *status = CacheStatus::Uninitialized;
                Ok(())
            }
            _ => Err(DeleterrError::new("Unable to access cache")),
        };
        cache_clear
    }
}
