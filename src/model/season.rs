use super::Episodes;

#[derive(Debug, Clone)]
pub struct Season {
    pub num: u32,
    pub episodes: Episodes,
}

#[derive(Debug)]
pub struct Seasons(pub Vec<Season>);
