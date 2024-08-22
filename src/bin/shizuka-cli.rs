use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Dialog, LinearLayout, SelectView};
use cursive::CursiveExt;
use cursive::{views::EditView, Cursive};
use shizuka_rs::model::Subtitles;
use std::sync::Arc;
use tokio::task::JoinHandle;

use shizuka_rs::{
    model::{Media, MediaKind, Season, Video},
    source::Source,
    sources::Cinemana,
};
use tokio::runtime::Runtime;

struct AppState {
    pub runtime: Runtime,
    pub client: Cinemana,
    pub mpv_proc_handle: Option<JoinHandle<()>>,
}

impl AppState {
    pub fn initialize() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let client = Cinemana::new().unwrap();

        Self {
            runtime,
            client,
            mpv_proc_handle: None,
        }
    }
}

// #[tokio::main]
fn main() {
    let mut siv = Cursive::default();
    let state = AppState::initialize();

    siv.set_user_data(state);

    siv.add_global_callback('q', Cursive::quit);

    let main_view = Dialog::new()
        .title("counter")
        .content(
            LinearLayout::horizontal()
                .child(EditView::new().with_name("query_edit").min_width(25))
                .child(
                    SelectView::new()
                        .item("movies", MediaKind::Movies)
                        .item("series", MediaKind::Series)
                        .with_name("select_kind"),
                ), // ListView::new()
        )
        .button("search", |siv| {
            let query_edit = siv
                .find_name::<EditView>("query_edit")
                .expect("failed to find");
            let query = query_edit.get_content();

            let select_kind = siv
                .find_name::<SelectView<MediaKind>>("select_kind")
                .unwrap();
            let kind_arc = select_kind
                .selection()
                .unwrap_or(Arc::new(MediaKind::Movies));
            let kind = match kind_arc.as_ref().to_owned() {
                MediaKind::Movies => MediaKind::Movies,
                MediaKind::Series => MediaKind::Series,
                _ => MediaKind::Unknown,
            };

            search_media(siv, &query.to_owned(), kind);
        })
        .button("quit", |siv| siv.quit());

    siv.add_layer(main_view);
    siv.run();
}

fn play_video(s: &mut Cursive, video: &Video, subtitles: Subtitles) {
    let state: &mut AppState = s.user_data().unwrap();

    match &state.mpv_proc_handle {
        Some(handle) => handle.abort(),
        None => {}
    }

    let video_url = video.url.clone();
    let handle = state.runtime.spawn(async move {
        let mut command = tokio::process::Command::new("mpv");
        command.args([video_url.as_str(), "--terminal=no", "--fullscreen"]);
        for subtitle in subtitles.0.into_iter() {
            command.arg(format!("--sub-file={}", subtitle.url.clone()));
        }
        command.kill_on_drop(true).status().await.unwrap();
    });
    state.mpv_proc_handle = Some(handle);
}

fn play_media(s: &mut Cursive, media: &Media) {
    let state: &mut AppState = s.user_data().unwrap();

    let media_stuff = state.runtime.block_on(async {
        tokio::try_join!(
            state.client.get_videos(media),
            state.client.get_subtitles(media),
        )
    });

    if let Ok(stuff) = media_stuff {
        s.add_layer(
            Dialog::around(
                LinearLayout::horizontal().child(
                    SelectView::new()
                        .with_all(
                            stuff
                                .0
                                 .0
                                .into_iter()
                                .map(|video| (video.name.to_owned(), video)),
                        )
                        .on_submit(move |siv, video| {
                            play_video(siv, video, stuff.1.clone());
                        })
                        .with_name("select_video")
                        .scrollable(),
                ),
            )
            .title("select video")
            .dismiss_button("Cancel"),
        );
    } else {
        s.add_layer(
            Dialog::new()
                .dismiss_button("OK")
                .title("Could not pull videos or subtitles"),
        );
    }
}

fn search_media(s: &mut Cursive, query: &str, kind: MediaKind) {
    let state: &mut AppState = s.user_data().unwrap();
    let result = state.runtime.block_on(async {
        state
            .client
            .search(query.to_string(), kind, None)
            .await
            .unwrap()
    });

    if result.0.is_empty() {
        s.add_layer(Dialog::new().dismiss_button("OK").title("No media found"));
        return;
    }

    s.add_layer(
        Dialog::around(
            SelectView::new()
                .with_all(
                    result
                        .0
                        .into_iter()
                        .map(|media| (format!("{} ({})", media.title, media.year), media)),
                )
                .on_submit(|s, m| {
                    if m.kind == MediaKind::Movies {
                        play_media(s, m);
                    } else {
                        select_season(s, m);
                    }
                })
                .scrollable(),
        )
        .title("select media")
        .dismiss_button("Cancel"),
    );
}

fn select_season(s: &mut Cursive, media: &Media) {
    let state: &mut AppState = s.user_data().unwrap();
    let episodes = state
        .runtime
        .block_on(async { state.client.get_seasons(media).await.unwrap() });
    s.add_layer(
        Dialog::around(
            SelectView::new()
                .with_all(
                    episodes
                        .0
                        .into_iter()
                        .map(|season| (format!("season {}", season.num), season)),
                )
                .on_submit(select_episode)
                .scrollable(),
        )
        .dismiss_button("Cancel"),
    );
}

fn select_episode(s: &mut Cursive, season: &Season) {
    // let state: &mut AppState = s.user_data().unwrap();
    let season_copy = season.clone();
    s.add_layer(
        Dialog::around(
            SelectView::new()
                .with_all(
                    season_copy
                        .episodes
                        .0
                        .into_iter()
                        .map(|episode| (format!("episode {}", episode.num), episode)),
                )
                .on_submit(|s, episode| play_media(s, &episode.media))
                .scrollable(),
        )
        .dismiss_button("Cancel"),
    );
}
