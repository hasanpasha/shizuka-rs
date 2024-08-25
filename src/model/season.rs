use super::Episodes;

#[derive(Debug, Clone, Default)]
pub struct Season {
    pub num: u32,
    pub episodes: Episodes,
}

#[derive(Debug, Default)]
pub struct Seasons(pub Vec<Season>);
