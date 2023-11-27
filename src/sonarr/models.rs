use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub max_season_with_file: Option<usize>,
}

impl From<Option<Vec<Episode>>> for SonarrShow {
    fn from(all_episodes: Option<Vec<Episode>>) -> Self {
        let mut seasons: HashMap<usize, EpisodeSeason> = HashMap::new();
        let mut max_season_with_file: Option<usize> = None; // there may be no seasons yet, or it might be a movie

        if let Some(episodes) = all_episodes {
            for episode in episodes {
                if episode.season_number > max_season_with_file.unwrap_or(0) && episode.has_file {
                    max_season_with_file = Some(episode.season_number)
                }
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

        SonarrShow {
            seasons,
            max_season_with_file,
        }
    }
}
