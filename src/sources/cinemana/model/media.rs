use crate::model::Media as CrateMedia;
use crate::model::Medias as CrateMedias;
use serde::Deserialize;

use super::MediaKind;

#[derive(Deserialize, Clone, Debug)]
pub struct Media {
    #[serde(rename(deserialize = "nb"))]
    pub id: String,
    #[serde(rename(deserialize = "en_title"))]
    pub title: String,
    pub year: String,
    #[serde(rename(deserialize = "kind"))]
    pub kind: MediaKind,
    #[serde(rename(deserialize = "imgThumbObjUrl"))]
    thumb: String,
    #[serde(rename(deserialize = "imgMediumThumbObjUrl"))]
    poster: String,
}

impl From<Media> for CrateMedia {
    fn from(val: Media) -> Self {
        CrateMedia {
            id: val.id,
            title: val.title,
            year: val.year,
            kind: val.kind.into(),
            thumbnail_url: Some(val.thumb),
            poster_url: Some(val.poster),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Medias(pub Vec<Media>);

impl From<Medias> for CrateMedias {
    fn from(val: Medias) -> Self {
        CrateMedias(val.0.iter().map(|x| x.to_owned().into()).collect())
    }
}
