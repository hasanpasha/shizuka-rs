use cursive::view::Scrollable;
use cursive::views::{Dialog, SelectView};
use cursive::CursiveExt;
use cursive::{views::EditView, Cursive};

use shizuka_rs::{
    model::{Media, MediaKind, Video},
    source::Source,
    sources::Cinemana,
};
use tokio::runtime::Runtime;

struct AppState {
    pub runtime: Runtime,
    pub client: Cinemana,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .unwrap(),
            client: Cinemana::new().unwrap(),
        }
    }
}

fn main() {
    let mut siv = Cursive::default();
    let state = AppState::default();

    siv.set_user_data(state);

    let main_view = Dialog::new()
        .title("counter")
        .content(EditView::new().on_submit(search_media))
        .button("quit", |siv| siv.quit());

    siv.add_layer(main_view);
    siv.run();
}

fn play_video(s: &mut Cursive, video: &Video) {
    let state: &mut AppState = s.user_data().unwrap();
    state.runtime.block_on(async {
        tokio::process::Command::new("mpv")
            .arg(video.url.as_str())
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    })
}

fn get_videos(s: &mut Cursive, media: &Media) {
    let state: &mut AppState = s.user_data().unwrap();
    let result = state
        .runtime
        .block_on(async { state.client.get_videos(media).await.unwrap() });
    s.add_layer(
        SelectView::new()
            .with_all(
                result
                    .0
                    .into_iter()
                    .map(|video| (video.name.to_owned(), video)),
            )
            .on_submit(play_video)
            .scrollable(),
    );
}

fn search_media(s: &mut Cursive, query: &str) {
    let state: &mut AppState = s.user_data().unwrap();
    let result = state.runtime.block_on(async {
        state
            .client
            .search(query.to_string(), MediaKind::Movies, None)
            .await
            .unwrap()
    });
    s.add_layer(
        SelectView::new()
            .with_all(
                result
                    .0
                    .into_iter()
                    .map(|media| (format!("{} {}", media.title, media.year), media)),
            )
            .on_submit(get_videos)
            .scrollable(),
    );
}
