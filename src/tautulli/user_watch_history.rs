use std::collections::HashMap;

use super::models::ResponseData;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

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
}

pub trait ConvertToHashMapBySeason {
    fn get_all_or_none(self) -> Option<Vec<UserWatchHistory>>;
    fn to_season_hashmap(self) -> HashMap<(usize, usize), UserWatchHistory>;
}
impl ConvertToHashMapBySeason for Option<Vec<UserWatchHistory>> {
    fn get_all_or_none(self) -> Option<Vec<UserWatchHistory>> {
        self
    }
    fn to_season_hashmap(self) -> HashMap<(usize, usize), UserWatchHistory> {
        self.into_iter()
            .flat_map(|history| {
                history.into_iter().filter_map(|item| {
                    if let (Some(season_number), Some(episode_number)) =
                        (item.parent_media_index, item.media_index)
                    {
                        Some(((season_number, episode_number), item))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

pub trait GetFirstOrNone {
    fn get_first_or_none(self) -> Option<UserWatchHistory>;
}

pub trait GetAllOrNone {
    fn get_all_or_none(self) -> Option<Vec<UserWatchHistory>>;
}

impl GetFirstOrNone for Option<Vec<UserWatchHistory>> {
    fn get_first_or_none(self) -> Option<UserWatchHistory> {
        match self {
            None => return None,
            Some(resp_data) => {
                if resp_data.len() == 1 {
                    Some(resp_data[0].clone())
                } else {
                    /* ! We return nothing if there is more than one result
                     * We make sure that there is exactly one matched result since
                     * we provided both a ratingKey and userId. If there is more than one result
                     * then we did something wrong.
                     */
                    None
                }
            }
        }
    }
}

impl GetAllOrNone for Option<ResponseData> {
    fn get_all_or_none(self) -> Option<Vec<UserWatchHistory>> {
        match self {
            None => None,
            Some(resp_data) => resp_data.data,
        }
    }
}
