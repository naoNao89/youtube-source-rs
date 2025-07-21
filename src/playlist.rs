use crate::YoutubeAudioTrack;

#[derive(Debug, Clone)]
pub struct YoutubePlaylist {
    pub name: String,
    pub tracks: Vec<YoutubeAudioTrack>,
    pub selected_track: Option<usize>,
    pub is_search_result: bool,
}

impl YoutubePlaylist {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
            selected_track: None,
            is_search_result: false,
        }
    }

    pub fn with_tracks(name: String, tracks: Vec<YoutubeAudioTrack>) -> Self {
        Self {
            name,
            tracks,
            selected_track: None,
            is_search_result: false,
        }
    }

    pub fn add_track(&mut self, track: YoutubeAudioTrack) {
        self.tracks.push(track);
    }

    pub fn set_selected_track(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.selected_track = Some(index);
        }
    }

    pub fn get_selected_track(&self) -> Option<&YoutubeAudioTrack> {
        self.selected_track.and_then(|index| self.tracks.get(index))
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub author: String,
    pub track_count: Option<usize>,
    pub thumbnail: Option<String>,
}
