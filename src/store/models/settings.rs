use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PurgeMarker {
    Watched,
    Time,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TVWatchedMarker {
    InProgress,
    Watched,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TVPurgeStrategy {
    Season,
    Show,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    tv_purge_marker: PurgeMarker,
    tv_watched_marker: Option<TVWatchedMarker>,
    tv_purge_delay: Option<usize>,
    tv_purge_strategy: TVPurgeStrategy,
    movie_purge_marker: PurgeMarker,
    movie_purge_delay: Option<usize>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tv_purge_marker: PurgeMarker::Watched,
            tv_watched_marker: None,
            tv_purge_delay: None,
            tv_purge_strategy: TVPurgeStrategy::Season,
            movie_purge_marker: PurgeMarker::Watched,
            movie_purge_delay: None,
        }
    }
}

impl Settings {
    pub fn as_vec(&self) -> Vec<u8> {
        let settings_bytes =
            serde_json::to_vec(&self).expect("Failed to serialize settings to JSON");
        settings_bytes
    }
}

impl From<Vec<u8>> for Settings {
    fn from(bytes: Vec<u8>) -> Self {
        let settings: Settings =
            serde_json::from_slice(&bytes).expect("Failed to deserialize settings");

        settings
    }
}
