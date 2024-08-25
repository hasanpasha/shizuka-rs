use super::Media;

#[derive(Debug, Clone, Default)]
pub struct Episode {
    pub num: u32,
    pub media: Media,
}

#[derive(Debug, Clone, Default)]
pub struct Episodes(pub Vec<Episode>);
