use crate::model::{Subtitle as CrateSubtitle, Subtitles as CrateSubtitles};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Subtitle {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "extention"))]
    pub extension: String,
    #[serde(rename(deserialize = "file"))]
    pub url: String,
}

impl From<Subtitle> for CrateSubtitle {
    fn from(val: Subtitle) -> Self {
        CrateSubtitle {
            name: val.name,
            extension: val.extension,
            url: val.url,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Subtitles {
    #[serde(default)]
    pub translations: Vec<Subtitle>,
}

impl From<Subtitles> for CrateSubtitles {
    fn from(val: Subtitles) -> Self {
        CrateSubtitles(
            val.translations
                .iter()
                .map(|x| x.to_owned().into())
                .collect(),
        )
    }
}
