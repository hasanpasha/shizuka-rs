use crate::model::{Video as CrateVideo, Videos as CrateVideos};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Video {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "videoUrl"))]
    pub url: String,
}

impl From<Video> for CrateVideo {
    fn from(val: Video) -> Self {
        CrateVideo {
            name: val.name,
            url: val.url,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Videos(pub Vec<Video>);

impl From<Videos> for CrateVideos {
    fn from(val: Videos) -> Self {
        CrateVideos(val.0.iter().map(|x| x.to_owned().into()).collect())
    }
}
