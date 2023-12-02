use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub title: String,
    pub status: String,
    pub ended: bool,
    pub tvdb_id: usize,
    pub id: usize,
    pub seasons: Vec<Season>,
    pub statistics: SeriesStatistics,
    pub images: Vec<Image>,
    pub first_aired: String,
}

impl Series {
    pub fn hashmap_seasons(series: &Option<Self>) -> HashMap<usize, Season> {
        match series {
            Some(series) => {
                let mut series_hashmap: HashMap<usize, Season> =
                    HashMap::with_capacity(series.statistics.season_count);

                for season in &series.seasons {
                    series_hashmap.insert(season.season_number, season.clone());
                }

                series_hashmap
            }
            None => {
                let series_hashmap: HashMap<usize, Season> = HashMap::with_capacity(0);
                series_hashmap
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub season_number: usize,
    pub monitored: bool,
    pub statistics: Statistics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub next_airing: Option<String>,
    pub previous_airing: Option<String>,
    pub episode_file_count: usize,
    pub episode_count: usize,
    pub total_episode_count: usize,
    pub size_on_disk: usize,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub cover_type: String,
    pub url: String,
    pub remote_url: String,
}
