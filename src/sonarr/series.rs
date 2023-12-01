use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub title: String,
    pub status: String,
    pub ended: bool,
    pub tvdb_id: usize,
    pub id: usize,
    pub seasons: Vec<Season>,
    pub statistics: SeriesStatistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub season_number: usize,
    pub monitored: bool,
    pub statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub next_airing: String,
    pub previous_airing: String,
    pub episode_file_count: usize,
    pub episode_count: usize,
    pub total_episode_count: usize,
    pub size_on_disk: usize,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatistics {
    pub season_count: usize,
    pub episode_file_count: usize,
    pub episode_count: usize,
    pub total_episode_count: usize,
    pub size_on_disk: usize,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: f64,
}
