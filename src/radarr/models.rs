use crate::sonrad::models::Image;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub title: String,
    pub sort_title: String,
    pub size_on_disk: usize,
    pub has_file: bool,
    pub movie_file: MovieFile,
    pub id: usize,
    pub images: Option<Vec<Image>>,
    pub digital_release: Option<String>,
    pub physical_release: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
    pub movie_id: usize,
    pub size: usize,
    pub date_added: String,
    pub indexer_flags: usize,
    pub edition: String,
    pub id: usize,
}
