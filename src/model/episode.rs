
#[derive(Debug)]
pub struct Episode {
    pub num: u32,
    pub id: String,
}

#[derive(Debug, Default)]
pub struct Episodes(pub Vec<Episode>);