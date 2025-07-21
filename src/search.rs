use crate::{YoutubeAudioTrack, YoutubePlaylist};
use std::time::Duration;

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

/// Represents different types of search results from YouTube
#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    /// A video search result
    Video {
        video_id: String,
        title: String,
        author: String,
        duration: Duration,
        uri: String,
    },
    /// A playlist search result
    Playlist {
        playlist_id: String,
        title: String,
        author: String,
        video_count: u32,
        uri: String,
    },
    /// A channel search result
    Channel {
        channel_id: String,
        title: String,
        subscriber_count: String,
        uri: String,
    },
}

impl SearchResult {
    /// Get the title of the search result
    pub fn title(&self) -> &str {
        match self {
            SearchResult::Video { title, .. } => title,
            SearchResult::Playlist { title, .. } => title,
            SearchResult::Channel { title, .. } => title,
        }
    }

    /// Get the URI of the search result
    pub fn uri(&self) -> &str {
        match self {
            SearchResult::Video { uri, .. } => uri,
            SearchResult::Playlist { uri, .. } => uri,
            SearchResult::Channel { uri, .. } => uri,
        }
    }

    /// Get the author/creator of the search result
    pub fn author(&self) -> &str {
        match self {
            SearchResult::Video { author, .. } => author,
            SearchResult::Playlist { author, .. } => author,
            SearchResult::Channel { title, .. } => title, // For channels, the title is the author
        }
    }

    /// Get the type of search result as a string
    pub fn result_type(&self) -> &'static str {
        match self {
            SearchResult::Video { .. } => "video",
            SearchResult::Playlist { .. } => "playlist",
            SearchResult::Channel { .. } => "channel",
        }
    }

    /// Check if this is a video result
    pub fn is_video(&self) -> bool {
        matches!(self, SearchResult::Video { .. })
    }

    /// Get the video ID if this is a video result
    pub fn video_id(&self) -> Option<&str> {
        match self {
            SearchResult::Video { video_id, .. } => Some(video_id),
            _ => None,
        }
    }

    /// Get the duration if this is a video result
    pub fn duration(&self) -> Option<Duration> {
        match self {
            SearchResult::Video { duration, .. } => Some(*duration),
            _ => None,
        }
    }
}
