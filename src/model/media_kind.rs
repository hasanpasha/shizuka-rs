#[derive(Debug, Clone, PartialEq, Default)]
pub enum MediaKind {
    #[default]
    Movies,
    Series,
    Unknown,
}
