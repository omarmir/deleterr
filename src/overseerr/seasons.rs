use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllSeasons {
    pub number_of_episodes: usize,
    pub number_of_seasons: usize,
    pub seasons: Vec<Season>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    season_number: usize,
    episode_count: usize,
}
