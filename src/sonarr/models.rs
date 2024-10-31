use serde::{Deserialize, Serialize};

/// Episode files for a series
///
/// # Properties:
///
/// * `series_id`: The `series_id` in Sonarr
/// * `season_number`: The `season_number` of the file
/// * `id`: The `id` of the file
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesEpisodes {
    pub series_id: usize,
    pub season_number: usize,
    pub size: usize,
    pub id: usize,
}
