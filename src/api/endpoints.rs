/// YouTube InnerTube API endpoints
pub struct YoutubeEndpoints;

impl YoutubeEndpoints {
    /// Base URL for YouTube InnerTube API
    pub const BASE_URL: &'static str = "https://www.youtube.com/youtubei/v1";

    /// Player endpoint for getting video information and formats
    pub const PLAYER: &'static str = "/player";

    /// Search endpoint for searching videos
    pub const SEARCH: &'static str = "/search";

    /// Browse endpoint for playlists and channels
    pub const BROWSE: &'static str = "/browse";

    /// Next endpoint for getting related videos and comments
    pub const NEXT: &'static str = "/next";

    /// Music search endpoint (music.youtube.com)
    pub const MUSIC_SEARCH_URL: &'static str = "https://music.youtube.com/youtubei/v1/search";

    /// Music search parameters
    pub const MUSIC_SEARCH_PARAMS: &'static str = "Eg-KAQwIARAAGAAgACgAMABqChAEEAUQAxAKEAk%3D";

    /// Get full URL for an endpoint
    pub fn get_url(endpoint: &str) -> String {
        format!("{}{}", Self::BASE_URL, endpoint)
    }

    /// Get player URL with API key
    pub fn get_player_url(api_key: &str) -> String {
        format!("{}{}?key={}", Self::BASE_URL, Self::PLAYER, api_key)
    }

    /// Get search URL with API key
    pub fn get_search_url(api_key: &str) -> String {
        format!("{}{}?key={}", Self::BASE_URL, Self::SEARCH, api_key)
    }

    /// Get browse URL with API key
    pub fn get_browse_url(api_key: &str) -> String {
        format!("{}{}?key={}", Self::BASE_URL, Self::BROWSE, api_key)
    }
}

/// Client configuration constants
pub struct ClientConstants;

impl ClientConstants {
    /// Default API key (extracted from YouTube homepage)
    pub const DEFAULT_API_KEY: &'static str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

    /// Web client name
    pub const WEB_CLIENT_NAME: &'static str = "WEB";

    /// Web client version
    pub const WEB_CLIENT_VERSION: &'static str = "2.20250403.01.00";

    /// Music client name
    pub const MUSIC_CLIENT_NAME: &'static str = "WEB_REMIX";

    /// Music client version
    pub const MUSIC_CLIENT_VERSION: &'static str = "1.20240724.00.00";

    /// Android client name
    pub const ANDROID_CLIENT_NAME: &'static str = "ANDROID";

    /// Android client version
    pub const ANDROID_CLIENT_VERSION: &'static str = "19.44.38";

    /// Default user agent for web client
    pub const WEB_USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

    /// Default user agent for Android client
    pub const ANDROID_USER_AGENT: &'static str =
        "com.google.android.youtube/19.44.38 (Linux; U; Android 11) gzip";
}
