use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: usize,
    pub episode_file_id: usize, //0 means no file - why is this not null I don't know
    pub series_id: usize,
    pub tvdb_id: usize,
    pub episode_number: usize,
    pub season_number: usize,
    pub has_file: bool,
}

pub struct EpisodeSeason {
    pub season_number: usize,
    pub episode_count: usize,
    pub episodes: HashMap<usize, Episode>,
}

pub struct SonarrShow {
    pub seasons: HashMap<usize, EpisodeSeason>,
}

impl From<Option<Vec<Episode>>> for SonarrShow {
    fn from(all_episodes: Option<Vec<Episode>>) -> Self {
        let mut seasons: HashMap<usize, EpisodeSeason> = HashMap::new();

        if let Some(episodes) = all_episodes {
            for episode in episodes {
                seasons
                    .entry(episode.season_number)
                    .and_modify(|episode_season| {
                        episode_season
                            .episodes
                            .insert(episode.episode_number, episode.clone());
                        episode_season.episode_count += 1;
                    })
                    .or_insert(EpisodeSeason {
                        season_number: episode.season_number,
                        episode_count: 1,
                        episodes: HashMap::from([(episode.episode_number, episode)]),
                    });
            }
        }

        SonarrShow { seasons }
    }
}
