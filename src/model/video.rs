#[derive(Debug)]
pub struct Video {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Videos(pub Vec<Video>);