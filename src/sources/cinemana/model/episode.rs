use crate::model::{
    Episode as CrateEpisode, Episodes as CrateEpisodes, Media, MediaKind, Season as CrateSeason,
    Seasons as CrateSeasons,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Episode {
    nb: String,
    season: String,
    #[serde(rename(deserialize = "episodeNummer"))]
    episode_number: String,
    year: String,
    en_title: String,
}

impl From<Episode> for CrateEpisode {
    fn from(val: Episode) -> Self {
        CrateEpisode {
            num: val.episode_number.parse().unwrap_or(0),
            media: Media {
                id: val.nb,
                title: val.en_title,
                year: val.year,
                kind: MediaKind::Series,
            },
        }
    }
}

impl From<Episode> for CrateSeason {
    fn from(val: Episode) -> Self {
        CrateSeason {
            num: val.season.parse().unwrap_or(0),
            episodes: CrateEpisodes::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Episodes(pub Vec<Episode>);

impl From<Episodes> for CrateSeasons {
    fn from(val: Episodes) -> Self {
        let mut seasons: Vec<CrateSeason> = val.0.iter().map(|x| x.to_owned().into()).collect();
        seasons.sort_by(|a, b| a.num.cmp(&b.num));
        seasons.dedup_by(|a, b| a.num == b.num);

        for season in seasons.iter_mut() {
            for episode in val.0.iter() {
                if episode.season.parse().unwrap_or(0) == season.num {
                    season.episodes.0.push(episode.to_owned().into());
                }
            }
        }
        CrateSeasons(seasons)
    }
}
