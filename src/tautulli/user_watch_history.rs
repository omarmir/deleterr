use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserWatchHistory {
    user: String,
    friendly_name: String,
    user_id: usize, // same is plex_id on tautulli
    full_title: String,
    pub watched_status: f32, //0: Unwatched or less than half, 0.5: watched more than 50%, and 1: Watched
    pub rating_key: usize,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    parent_rating_key: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    grandparent_rating_key: Option<usize>,
    user_thumb: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub media_index: Option<usize>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub parent_media_index: Option<usize>,
    pub originally_available_at: Option<String>,
}

pub trait ConvertToHashMapBySeason {
    fn hashmap_seasons(self) -> HashMap<usize, Vec<UserWatchHistory>>;
}
impl ConvertToHashMapBySeason for Option<Vec<UserWatchHistory>> {
    fn hashmap_seasons(self) -> HashMap<usize, Vec<UserWatchHistory>> {
        let mut episode_map: HashMap<(usize, usize), UserWatchHistory> = HashMap::new();
        let mut season_map: HashMap<usize, Vec<UserWatchHistory>> = HashMap::new();

        if let Some(histories) = self {
            for item in histories {
                if let (Some(season_number), Some(episode_number)) =
                    (item.parent_media_index, item.media_index)
                {
                    // One episode can have more than one watch history - I dont know why but I've seen it.
                    episode_map
                        .entry((season_number, episode_number))
                        .and_modify(|existing_value| {
                            if existing_value.watched_status < item.watched_status {
                                *existing_value = item.clone();
                            }
                        })
                        .or_insert(item);
                }
            }
        }

        for ((key1, _key2), value) in episode_map {
            season_map.entry(key1).or_insert_with(Vec::new).push(value);
        }

        season_map
    }
}

pub trait GetFirstOrNone {
    fn get_first_or_none(self) -> Option<UserWatchHistory>;
}

impl GetFirstOrNone for Option<Vec<UserWatchHistory>> {
    fn get_first_or_none(self) -> Option<UserWatchHistory> {
        match self {
            None => return None,
            Some(resp_data) => {
                // So apparently you can have more than one watch result per user for the same file. Pick the most watched session.
                let max_watched = resp_data.iter().max_by(|a, b| {
                    a.watched_status
                        .partial_cmp(&b.watched_status)
                        .unwrap_or(Ordering::Equal)
                });

                max_watched.cloned()
            }
        }
    }
}
