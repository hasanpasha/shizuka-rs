use log::info;

use shizuka_rs::{source::Source, sources::Cinemana, model::MediaKind};

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );

    let cinemana = Cinemana::new().expect("failed to create cinemana");
    let result = cinemana.search("naruto".to_string(), MediaKind::Series, Some(0)).await.expect("failed to get result");
    info!("search result: {:?}", result);

    let first_media = result.0.first().unwrap();

    let videos = cinemana.get_videos(first_media).await.expect("failed to get videos");
    info!("first result videos: {:?}", videos);

    let subtitles = cinemana.get_subtitles(first_media).await.expect("failed to get subtitle");
    info!("subtitles: {:?}", subtitles);

    let seasons = cinemana.get_seasons(first_media).await.expect("failed to get episodes");
    info!("seasons: {:?}", seasons);
}