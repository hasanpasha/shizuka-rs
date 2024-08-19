use serde::Deserialize;
use crate::model::{
    Subtitle as CrateSubtitle,
    Subtitles as CrateSubtitles
};

#[derive(Debug, Clone, Deserialize)]
pub struct Subtitle {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "extention"))]
    pub extension: String,
    #[serde(rename(deserialize = "file"))]
    pub url: String,
}

impl Into<CrateSubtitle> for Subtitle {
    fn into(self) -> CrateSubtitle {
        CrateSubtitle { name: self.name, extension: self.extension, url: self.url }
    }
}

#[derive(Deserialize, Debug)]
pub struct Subtitles{
    #[serde(default)]
    pub translations: Vec<Subtitle>
}

impl Into<CrateSubtitles> for Subtitles {
    fn into(self) -> CrateSubtitles {
        CrateSubtitles(self.translations.iter().map(|x| x.to_owned().into()).collect())
    }
}