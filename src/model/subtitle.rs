#[derive(Debug)]
pub struct Subtitle {
    pub name: String,
    pub extension: String,
    pub url: String,
}

// #[derive(Debug)]
// pub enum SubtitleExtension {
//     ASS,
//     SRT,
//     VVT,
//     UNKNOWN,
// }

#[derive(Debug)]
pub struct Subtitles(pub Vec<Subtitle>);