#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaKind {
    #[default]
    Movies,
    Series,
}

impl MediaKind {
    const ALL: [MediaKind; 2] = [MediaKind::Movies, MediaKind::Series];
}

impl std::fmt::Display for MediaKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MediaKind::Movies => "Movies",
                MediaKind::Series => "Series",
            }
        )
    }
}
