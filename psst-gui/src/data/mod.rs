mod album;
mod artist;
mod config;
mod ctx;
mod promise;
mod route;
mod track;
mod utils;

pub use crate::data::{
    album::{Album, AlbumType},
    artist::Artist,
    config::{AudioQuality, Config},
    ctx::Ctx,
    promise::{Promise, PromiseState},
    route::{Navigation, Route},
    track::{Track, TrackCtx, TrackId},
    utils::{AudioDuration, Image},
};

use druid::{
    im::{HashSet, Vector},
    Data, Lens,
};
use std::sync::Arc;

#[derive(Clone, Debug, Data, Lens)]
pub struct State {
    pub route: Route,
    pub history: Vector<Navigation>,
    pub config: Config,
    pub playback: Playback,
    pub search: Search,
    pub album: AlbumDetail,
    pub artist: ArtistDetail,
    pub playlist: PlaylistDetail,
    pub library: Library,
    pub track_ctx: TrackCtx,
}

impl Default for State {
    fn default() -> Self {
        Self {
            route: Route::Home,
            history: Vector::new(),
            config: Config::default(),
            playback: Playback {
                is_playing: false,
                progress: None,
                item: None,
            },
            search: Search {
                input: String::new(),
                results: Promise::Empty,
            },
            album: AlbumDetail {
                id: String::new(),
                album: Promise::Empty,
            },
            artist: ArtistDetail {
                id: String::new(),
                artist: Promise::Empty,
                albums: Promise::Empty,
                top_tracks: Promise::Empty,
            },
            playlist: PlaylistDetail {
                playlist: Promise::Empty,
                tracks: Promise::Empty,
            },
            library: Library {
                saved_albums: Promise::Empty,
                saved_tracks: Promise::Empty,
                playlists: Promise::Empty,
            },
            track_ctx: TrackCtx {
                playback_item: None,
                saved_tracks: HashSet::new(),
            },
        }
    }
}

impl State {
    pub fn set_playback_playing(&mut self, item: Arc<Track>) {
        self.playback.is_playing = true;
        self.playback.item.replace(item.clone());
        self.playback.progress.take();
        self.track_ctx.playback_item.replace(item);
    }

    pub fn set_playback_progress(&mut self, progress: AudioDuration) {
        self.playback.progress.replace(progress);
    }

    pub fn set_playback_paused(&mut self) {
        self.playback.is_playing = false;
    }

    pub fn set_playback_stopped(&mut self) {
        self.playback.is_playing = false;
        self.playback.item.take();
        self.playback.progress.take();
        self.track_ctx.playback_item.take();
    }
}

#[derive(Clone, Debug, Data)]
pub struct PlaybackCtx {
    pub tracks: Vector<Arc<Track>>,
    pub position: usize,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct Playback {
    pub is_playing: bool,
    pub progress: Option<AudioDuration>,
    pub item: Option<Arc<Track>>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct Search {
    pub input: String,
    pub results: Promise<SearchResults, String>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct SearchResults {
    pub artists: Vector<Artist>,
    pub albums: Vector<Album>,
    pub tracks: Vector<Arc<Track>>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct Library {
    pub saved_albums: Promise<Vector<Album>>,
    pub saved_tracks: Promise<Vector<Arc<Track>>>,
    pub playlists: Promise<Vector<Playlist>>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct AlbumDetail {
    pub id: String,
    pub album: Promise<Album, String>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct ArtistDetail {
    pub id: String,
    pub artist: Promise<Artist, String>,
    pub albums: Promise<Vector<Album>, String>,
    pub top_tracks: Promise<Vector<Arc<Track>>, String>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct PlaylistDetail {
    pub playlist: Promise<Playlist>,
    pub tracks: Promise<Vector<Arc<Track>>, String>,
}

#[derive(Clone, Debug, Data, Lens)]
pub struct Playlist {
    pub id: String,
    pub images: Vector<Image>,
    pub name: String,
}