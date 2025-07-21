use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeSourceOptions {
    pub allow_search: bool,
    pub allow_direct_video_ids: bool,
    pub allow_direct_playlist_ids: bool,
}

impl Default for YoutubeSourceOptions {
    fn default() -> Self {
        Self {
            allow_search: true,
            allow_direct_video_ids: true,
            allow_direct_playlist_ids: true,
        }
    }
}

impl YoutubeSourceOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_allow_search(mut self, allow_search: bool) -> Self {
        self.allow_search = allow_search;
        self
    }

    pub fn set_allow_direct_video_ids(mut self, allow_direct_video_ids: bool) -> Self {
        self.allow_direct_video_ids = allow_direct_video_ids;
        self
    }

    pub fn set_allow_direct_playlist_ids(mut self, allow_direct_playlist_ids: bool) -> Self {
        self.allow_direct_playlist_ids = allow_direct_playlist_ids;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientOptions {
    pub playback: bool,
    pub playlist_loading: bool,
    pub video_loading: bool,
    pub searching: bool,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            playback: true,
            playlist_loading: true,
            video_loading: true,
            searching: true,
        }
    }
}

impl ClientOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_playback(mut self, playback: bool) -> Self {
        self.playback = playback;
        self
    }

    pub fn set_playlist_loading(mut self, playlist_loading: bool) -> Self {
        self.playlist_loading = playlist_loading;
        self
    }

    pub fn set_video_loading(mut self, video_loading: bool) -> Self {
        self.video_loading = video_loading;
        self
    }

    pub fn set_searching(mut self, searching: bool) -> Self {
        self.searching = searching;
        self
    }
}
