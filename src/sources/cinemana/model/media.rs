use serde::Deserialize;
use crate::model::Medias as CrateMedias;
use crate::model::Media as CrateMedia;

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
}

impl Into<CrateMedia> for Media {
    fn into(self: Media) -> CrateMedia {
        CrateMedia{ id: self.id, title: self.title, year: self.year, kind: self.kind.into() }
    }
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Medias(pub Vec<Media>);

impl Into<CrateMedias> for Medias {
    fn into(self) -> CrateMedias {
        CrateMedias(self.0.iter().map(|x| x.to_owned().into()).collect())
    }
}