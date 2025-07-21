use crate::{AudioItem, ClientOptions, Result, TrackFormats, YoutubeAudioSourceManager};
use async_trait::async_trait;

/// Comprehensive client capabilities structure
/// Maps directly to the Client Capabilities Summary table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCapabilities {
    /// Supports OAuth authentication
    pub oauth: bool,
    /// Can load individual videos
    pub videos: bool,
    /// Can load playlists (excluding mixes)
    pub playlists: bool,
    /// Can load mixes (RD playlists)
    pub mixes: bool,
    /// Can perform search operations
    pub search: bool,
    /// Can be used in embedded contexts
    pub embedded: bool,
}

// YouTube API constants
pub const WATCH_URL: &str = "https://www.youtube.com/watch?v=";
pub const API_BASE_URL: &str = "https://youtubei.googleapis.com/youtubei/v1";
pub const PLAYER_URL: &str = "https://youtubei.googleapis.com/youtubei/v1/player?prettyPrint=false";
pub const SEARCH_URL: &str = "https://youtubei.googleapis.com/youtubei/v1/search?prettyPrint=false";
pub const NEXT_URL: &str = "https://youtubei.googleapis.com/youtubei/v1/next?prettyPrint=false";
pub const BROWSE_URL: &str = "https://youtubei.googleapis.com/youtubei/v1/browse?prettyPrint=false";

pub const MUSIC_API_BASE_URL: &str = "https://music.youtube.com/youtubei/v1";
pub const MUSIC_SEARCH_URL: &str = "https://music.youtube.com/youtubei/v1/search?prettyPrint=false";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayabilityStatus {
    Ok,
    NonEmbeddable,
    RequiresLogin,
    PremiereTrailer,
}

#[async_trait]
pub trait Client: Send + Sync {
    fn get_identifier(&self) -> &'static str;
    fn get_player_params(&self) -> Option<&str> {
        None
    }
    fn get_options(&self) -> &ClientOptions;
    fn can_handle_request(&self, identifier: &str) -> bool;
    fn supports_format_loading(&self) -> bool {
        self.get_options().playback
    }
    fn is_embedded(&self) -> bool {
        self.get_capabilities().embedded
    }
    fn supports_oauth(&self) -> bool {
        self.get_capabilities().oauth
    }
    fn requires_player_script(&self) -> bool {
        true
    }

    /// Get comprehensive capabilities for this client
    fn get_capabilities(&self) -> ClientCapabilities;

    /// Check if client supports video loading
    fn supports_videos(&self) -> bool {
        self.get_capabilities().videos
    }

    /// Check if client supports playlist loading (excluding mixes)
    fn supports_playlists(&self) -> bool {
        self.get_capabilities().playlists
    }

    /// Check if client supports mix loading
    fn supports_mixes(&self) -> bool {
        self.get_capabilities().mixes
    }

    /// Check if client supports search operations
    fn supports_search(&self) -> bool {
        self.get_capabilities().search
    }

    async fn load_video(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>>;

    async fn load_playlist(
        &self,
        source: &YoutubeAudioSourceManager,
        playlist_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>>;

    async fn search(
        &self,
        source: &YoutubeAudioSourceManager,
        query: &str,
    ) -> Result<Option<AudioItem>>;

    async fn get_track_formats(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<TrackFormats>;

    async fn load_mix(
        &self,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>>;

    fn transform_playback_uri(&self, _original: &url::Url, resolved: &url::Url) -> url::Url {
        resolved.clone()
    }

    /// Enable downcasting to concrete client types
    fn as_any(&self) -> &dyn std::any::Any;
}

impl ClientCapabilities {
    /// Create capabilities for Android Standard client
    pub fn android_standard() -> Self {
        Self {
            oauth: true,
            videos: true,
            playlists: true,
            mixes: true,
            search: true,
            embedded: false,
        }
    }

    /// Create capabilities for Android Music client
    pub fn android_music() -> Self {
        Self {
            oauth: true,
            videos: true,
            playlists: false,
            mixes: true,
            search: true,
            embedded: false,
        }
    }

    /// Create capabilities for Android VR client
    pub fn android_vr() -> Self {
        Self {
            oauth: true,
            videos: true,
            playlists: true,
            mixes: true,
            search: true,
            embedded: false,
        }
    }

    /// Create capabilities for iOS client
    pub fn ios() -> Self {
        Self {
            oauth: true,
            videos: true,
            playlists: false,
            mixes: true,
            search: true,
            embedded: false,
        }
    }

    /// Create capabilities for TV Standard client
    pub fn tv_standard() -> Self {
        Self {
            oauth: true,
            videos: false,
            playlists: false,
            mixes: false,
            search: true,
            embedded: false,
        }
    }

    /// Create capabilities for TV HTML5 Embedded client
    pub fn tv_html5_embedded() -> Self {
        Self {
            oauth: true,
            videos: true,
            playlists: false,
            mixes: true,
            search: true,
            embedded: true,
        }
    }
}

/// Generate a capabilities summary table for all client types
/// Matches the format of the Client Capabilities Summary table
pub fn generate_capabilities_summary() -> String {
    let clients = vec![
        ("Android Standard", ClientCapabilities::android_standard()),
        ("Android Music", ClientCapabilities::android_music()),
        ("Android VR", ClientCapabilities::android_vr()),
        ("iOS", ClientCapabilities::ios()),
        ("TV Standard", ClientCapabilities::tv_standard()),
        ("TV HTML5 Embedded", ClientCapabilities::tv_html5_embedded()),
    ];

    let mut summary = String::from("Client Capabilities Summary\n");
    summary.push_str("==============================\n");
    summary.push_str("| Client | OAuth | Videos | Playlists | Mixes | Search | Embedded |\n");
    summary.push_str("| ------ | ----- | ------ | --------- | ----- | ------ | -------- |\n");

    for (name, caps) in clients {
        summary.push_str(&format!(
            "| {:<15} | {} | {} | {} | {} | {} | {} |\n",
            name,
            if caps.oauth { "✅" } else { "❌" },
            if caps.videos { "✅" } else { "❌" },
            if caps.playlists { "✅" } else { "❌" },
            if caps.mixes { "✅" } else { "❌" },
            if caps.search { "✅" } else { "❌" },
            if caps.embedded { "✅" } else { "❌" },
        ));
    }

    summary
}
