use std::cmp::Ordering;

use serde::Deserialize;
use crate::model::{
    Episode as CrateEpisode,
    Episodes as CrateEpisodes,
    Season as CrateSeason,
    Seasons as CrateSeasons
};


#[derive(Debug, Clone, Deserialize)]
pub struct Episode {
    nb: String,
    season: String,
    #[serde(rename(deserialize = "episodeNummer"))]
    episode_number: String,
}

impl Into<CrateEpisode> for Episode {
    fn into(self) -> CrateEpisode {
        CrateEpisode { num: self.episode_number.parse().unwrap_or(0), id: self.nb }
    }
}

impl Into<CrateSeason> for Episode {
    fn into(self) -> CrateSeason {
        CrateSeason { num: self.season.parse().unwrap_or(0), episodes: CrateEpisodes::default() }
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Episodes(pub Vec<Episode>);

impl Into<CrateSeasons> for Episodes {
    fn into(self) -> CrateSeasons {
        let mut seasons: Vec<CrateSeason> = self.0.iter().map(|x| x.to_owned().into()).collect();
        seasons.sort_by(|a, b| cmp_seasons(a, b) );
        seasons.dedup_by(|a, b| a.num == b.num);

        for season in seasons.iter_mut() {
            for episode in self.0.iter() {
                if episode.season.parse().unwrap_or(0) == season.num {
                    season.episodes.0.push(episode.to_owned().into());
                }
            }
        }
        CrateSeasons(seasons)
    }
}

fn cmp_seasons(a: &CrateSeason, b: &CrateSeason) -> Ordering {
    if a.num > b.num {
        Ordering::Greater
    } else if a.num < b.num {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}