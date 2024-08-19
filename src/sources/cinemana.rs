use std::time::Duration;
use log::debug;
use reqwest::{Client, ClientBuilder, Url};
use serde::de::DeserializeOwned;

mod model;

use crate::source::{Source, SourceResult};
use model::{Episodes, MediaKind, Medias, Subtitles, Videos};

use crate::model as crate_model;

const API_BASE: &str = "https://cinemana.shabakaty.com/api/android/";

pub struct Cinemana {
    client: Client,
}

impl Cinemana {
    pub fn new() -> reqwest::Result<Self> {
        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Cinemana{
            client: http_client
        })
    }
}

impl Source for Cinemana {
    async fn search(&self, query: String, kind: crate_model::MediaKind, page: Option<u32>) -> SourceResult<crate_model::Medias> {
        let mut endpoint = Url::parse(API_BASE)?
            .join("AdvancedSearch")?;
        endpoint.set_query(Some(format!("videoTitle={}&type={}&page={}", query, MediaKind::from(kind), page.unwrap_or(0)).as_str()));

        debug!("fetching: {}", endpoint);
        let result = fetch::<Medias>(&self.client, endpoint).await?;
        Ok(result.into())
    }

    async fn get_videos(&self, media: &crate_model::Media) -> SourceResult<crate_model::Videos> {
        let endpoint = Url::parse(API_BASE)?
            .join(format!("transcoddedFiles/id/{}", media.id).as_str())?;

        debug!("fetching: {}", endpoint);
        let result = fetch::<Videos>(&self.client, endpoint).await?;
        Ok(result.into())
    }

    async fn get_subtitles(&self, media: &crate_model::Media) -> SourceResult<crate_model::Subtitles> {
        let endpoint = Url::parse(API_BASE)?
            .join(format!("translationFiles/id/{}", media.id).as_str())?;

        debug!("fetching: {}", endpoint);
        let result = fetch::<Subtitles>(&self.client, endpoint).await?;
        Ok(result.into())
    }

    async fn get_seasons(&self, media: &crate_model::Media) -> SourceResult<crate_model::Seasons> {
        assert!(media.kind == crate_model::MediaKind::Series);

        let endpoint = Url::parse(API_BASE)?
            .join(format!("videoSeason/id/{}", media.id).as_str())?;

        debug!("fetching: {}", endpoint);
        let result = fetch::<Episodes>(&self.client, endpoint).await?;

        Ok(result.into())
    }
}

async fn fetch<T: DeserializeOwned>(client: &Client, endpoint: Url) -> reqwest::Result<T> {
    let result = client.get(endpoint).send().await?.json::<T>().await?;
    return Ok(result);
}