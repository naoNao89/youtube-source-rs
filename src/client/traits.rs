use async_trait::async_trait;
use crate::{AudioItem, YoutubeAudioSourceManager, TrackFormats, ClientOptions, Result};

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
    fn get_player_params(&self) -> Option<&str> { None }
    fn get_options(&self) -> &ClientOptions;
    fn can_handle_request(&self, identifier: &str) -> bool;
    fn supports_format_loading(&self) -> bool { self.get_options().playback }
    fn is_embedded(&self) -> bool { false }
    fn supports_oauth(&self) -> bool { false }
    fn requires_player_script(&self) -> bool { true }

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

    fn transform_playback_uri(&self, original: &url::Url, resolved: &url::Url) -> url::Url {
        resolved.clone()
    }
}
