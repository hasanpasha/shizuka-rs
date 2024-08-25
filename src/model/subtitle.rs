#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct Subtitles(pub Vec<Subtitle>);
