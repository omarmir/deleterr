use crate::sonrad::models::Image;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub title: String,
    pub status: String,
    pub sort_title: String,
    pub ended: bool,
    pub tvdb_id: usize,
    pub id: usize,
    pub seasons: Vec<Season>,
    pub statistics: SeriesStatistics,
    pub images: Option<Vec<Image>>,
    pub first_aired: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
pub enum FinaleType {
    Season,
    Series,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Episode {
    series_id: u32,
    tvdb_id: u64,
    episode_file_id: u32,
    season_number: u32,
    episode_number: u32,
    finale_type: Option<FinaleType>,
    has_file: bool,
    monitored: bool,
    id: u32,
}
