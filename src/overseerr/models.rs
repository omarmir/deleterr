use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct OverseerrRequestsCount {
    total: u32,
    movie: u32,
    tv: u32,
    pending: u32,
    approved: u32,
    declined: u32,
    processing: u32,
    pub available: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OverseerrListResponse<T> {
    pub page_info: PageInfo,
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageInfo {
    pub page: u32,
    pub pages: usize,
    pub results: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaRequest {
    pub id: usize,
    pub status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
    pub requested_by: User,
    pub media: Media,
    pub created_at: DateTime<Utc>,
    pub season_count: Option<usize>,
    #[serde(skip_serializing)]
    pub seasons: Vec<RequestSeason>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestSeason {
    pub id: usize,
    pub season_number: usize,
    pub status: u8, // Status of the request. 1 = PENDING APPROVAL, 2 = APPROVED, 3 = DECLINED
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllSeasons {
    pub number_of_episodes: usize,
    pub number_of_seasons: usize,
    pub seasons: Vec<AllSeasonsSeason>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllSeasonsSeason {
    pub id: usize,
    pub season_number: usize,
    name: Option<String>,
    pub episode_count: usize,
}

impl AllSeasons {
    pub fn movie_season() -> Self {
        AllSeasons {
            number_of_episodes: 1,
            number_of_seasons: 1,
            seasons: Vec::from([AllSeasonsSeason {
                id: 0,
                season_number: 1,
                episode_count: 1,
                name: None,
            }]),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    pub username: Option<String>,
    user_type: u32,
    email: Option<String>,
    plex_username: Option<String>,
    pub plex_id: Option<u64>, // Don't know if its 32 bit or 64 - same as tautulli user
    avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: Option<usize>,
    pub media_type: MediaType,
    pub tmdb_id: Option<usize>,
    tvdb_id: Option<usize>, // Apparently nothing uses this. Even if it says tvid in the API.
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub rating_key: Option<u64>,
    pub external_service_id: Option<usize>,
    pub status: u8, // Availability of the media. 1 = UNKNOWN, 2 = PENDING, 3 = PROCESSING, 4 = PARTIALLY_AVAILABLE, 5 = AVAILABLE
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    TV,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub poster_path: Option<String>,
    #[serde(alias = "firstAirDate")]
    pub release_date: Option<String>, //firstAirDate
    #[serde(alias = "name")]
    pub title: String, //name
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediaInfoQueryParms {
    pub media_type: MediaType,
    pub id: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AboutServer {
    version: String,
    total_requests: usize,
    total_media_items: usize,
    tz: String,
    app_data_path: String,
}
