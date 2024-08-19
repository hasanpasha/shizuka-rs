use std::{error::Error, future::Future};

use crate::model::{Media, Medias, MediaKind, Videos, Subtitles, Seasons};

pub trait Source {
    fn search(&self, query: String, kind: MediaKind, page: Option<u32>) -> impl Future<Output = SourceResult<Medias>> + Send;
    fn get_seasons(&self, media: &Media) -> impl Future<Output = SourceResult<Seasons>> + Send;
    fn get_videos(&self, media: &Media) -> impl Future<Output = SourceResult<Videos>> + Send;
    fn get_subtitles(&self, media: &Media) -> impl Future<Output = SourceResult<Subtitles>> + Send;
}

// pub struct SourceError;

pub type BoxError = Box<dyn Error>;

pub type SourceResult<T> = std::result::Result<T, BoxError>;