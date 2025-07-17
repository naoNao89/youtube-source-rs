use crate::{YoutubeAudioTrack, YoutubePlaylist};

#[derive(Debug, Clone)]
pub struct YoutubeSearchResult {
    pub tracks: Vec<YoutubeAudioTrack>,
    pub playlists: Vec<YoutubePlaylist>,
    pub query: String,
}

impl YoutubeSearchResult {
    pub fn new(query: String) -> Self {
        Self {
            tracks: Vec::new(),
            playlists: Vec::new(),
            query,
        }
    }

    pub fn with_tracks(query: String, tracks: Vec<YoutubeAudioTrack>) -> Self {
        Self {
            tracks,
            playlists: Vec::new(),
            query,
        }
    }

    pub fn add_track(&mut self, track: YoutubeAudioTrack) {
        self.tracks.push(track);
    }

    pub fn add_playlist(&mut self, playlist: YoutubePlaylist) {
        self.playlists.push(playlist);
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty() && self.playlists.is_empty()
    }

    pub fn total_results(&self) -> usize {
        self.tracks.len() + self.playlists.len()
    }
}
