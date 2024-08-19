use serde::Deserialize;
use crate::model::{Video as CrateVideo, Videos as CrateVideos};

#[derive(Deserialize, Clone, Debug)]
pub struct Video {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "videoUrl"))]
    pub url: String,
}

impl Into<CrateVideo> for Video {
    fn into(self) -> CrateVideo {
        CrateVideo{ name: self.name, url: self.url }
    }
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Videos(pub Vec<Video>);

impl Into<CrateVideos> for Videos {
    fn into(self) -> CrateVideos {
        CrateVideos(self.0.iter().map(|x| x.to_owned().into()).collect())
    }
}