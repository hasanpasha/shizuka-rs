#[derive(Debug, Clone, Default)]
pub struct Video {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Videos(pub Vec<Video>);
