use super::MediaKind;

#[derive(Debug, Clone, Default)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub year: String,
    pub kind: MediaKind,
}

impl PartialEq for Media {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct Medias(pub Vec<Media>);

// #[derive(Deserialize, Debug)]
// #[serde(transparent)]
// pub struct Medias {
//     pub medias: Vec<Media>,
// }
