use crate::client::config::ClientConfig;
use crate::client::traits::ClientCapabilities;
use crate::client::{Client, NonMusicClient, NonMusicClientBase};
use crate::http::YoutubeHttpClient;
use crate::playlist::YoutubePlaylist;
use crate::track::{AudioTrackInfo, TrackFormats};
use async_trait::async_trait;

use crate::config::ClientOptions;
use crate::error::Result;
use crate::{AudioItem, YoutubeAudioSourceManager};

/// Web client variants
#[derive(Debug, Clone, PartialEq)]
pub enum WebVariant {
    /// Standard Web client - migrated from Web.java
    Standard,
    /// Mobile Web client - migrated from MWeb.java
    Mobile,
}

/// YouTube Web Client implementation
///
/// Based on Java Web.java and MWeb.java, this is the primary client for most YouTube content.
/// Features:
/// - Standard YouTube web client behavior
/// - Mobile web client variant support
/// - Dynamic client configuration fetching from YouTube homepage
/// - PoToken and visitor data integration
/// - Web-specific JSON parsing for search and playlists
#[derive(Debug)]
pub struct WebClient {
    base: NonMusicClientBase,
    po_token: Option<String>,
    visitor_data: Option<String>,
}

impl WebClient {
    pub fn new() -> Result<Self> {
        let http_client = YoutubeHttpClient::new()?;
        let client_config = ClientConfig::web();
        let base = NonMusicClientBase::new(http_client, client_config, "WEB".to_string());

        Ok(Self {
            base,
            po_token: None,
            visitor_data: None,
        })
    }

    pub fn with_config(client_config: ClientConfig) -> Result<Self> {
        let http_client = YoutubeHttpClient::new()?;
        let base = NonMusicClientBase::new(http_client, client_config, "WEB".to_string());

        Ok(Self {
            base,
            po_token: None,
            visitor_data: None,
        })
    }

    /// Create Mobile Web client variant
    /// Migrated from MWeb.java
    pub fn mobile() -> Result<Self> {
        let http_client = YoutubeHttpClient::new()?;
        let client_config = ClientConfig::mobile_web();
        let base = NonMusicClientBase::new(http_client, client_config, "MWEB".to_string());

        Ok(Self {
            base,
            po_token: None,
            visitor_data: None,
        })
    }

    /// Create Mobile Web client with config
    pub fn mobile_with_config(client_config: ClientConfig) -> Result<Self> {
        let http_client = YoutubeHttpClient::new()?;
        let base = NonMusicClientBase::new(http_client, client_config, "MWEB".to_string());

        Ok(Self {
            base,
            po_token: None,
            visitor_data: None,
        })
    }

    /// Set PoToken and visitor data for enhanced access
    ///
    /// Based on Java Web.setPoTokenAndVisitorData() static method.
    /// This enables access to more content and reduces rate limiting.
    pub fn set_po_token_and_visitor_data(
        &mut self,
        po_token: Option<String>,
        visitor_data: Option<String>,
    ) {
        self.po_token = po_token;
        self.visitor_data = visitor_data.clone();

        // Update visitor data in HTTP filter
        if let Some(visitor_data) = visitor_data {
            tokio::spawn({
                let filter = self.base.get_http_client().filter().clone();
                async move {
                    filter.set_visitor_id(visitor_data).await;
                }
            });
        }
    }

    /// Fetch dynamic client configuration from YouTube homepage
    ///
    /// Based on Java Web.fetchClientConfig() method.
    /// This scrapes the YouTube homepage to get the latest client version and API key.
    pub async fn fetch_client_config(&self) -> Result<ClientConfig> {
        // TODO: Implement dynamic config fetching
        // For now, return static config
        Ok(ClientConfig::web())
    }
}

// Implement NonMusicClient trait by delegating to base
#[async_trait]
impl NonMusicClient for WebClient {
    async fn load_track_info_from_innertube(&self, video_id: &str) -> Result<AudioTrackInfo> {
        self.base.load_track_info_from_innertube(video_id).await
    }

    async fn load_search_results(&self, query: &str) -> Result<Vec<crate::search::SearchResult>> {
        self.base.load_search_results(query).await
    }

    async fn load_playlist(&self, playlist_id: &str) -> Result<YoutubePlaylist> {
        NonMusicClient::load_playlist(&self.base, playlist_id).await
    }

    fn get_http_client(&self) -> &YoutubeHttpClient {
        self.base.get_http_client()
    }

    fn get_client_config(&self) -> &ClientConfig {
        self.base.get_client_config()
    }
}

// Implement base Client trait by delegating to base
#[async_trait]
impl Client for WebClient {
    fn get_identifier(&self) -> &'static str {
        "WEB"
    }

    fn get_options(&self) -> &ClientOptions {
        self.base.get_options()
    }

    fn can_handle_request(&self, identifier: &str) -> bool {
        use crate::utils;

        // Can handle video IDs, YouTube URLs, and search queries
        utils::extract_video_id(identifier).is_some()
            || utils::extract_playlist_id(identifier).is_some()
            || identifier.contains("youtube.com")
            || identifier.contains("youtu.be")
    }

    fn get_capabilities(&self) -> ClientCapabilities {
        // Web clients support all features except embedded
        ClientCapabilities {
            oauth: true,
            videos: true,
            playlists: true,
            mixes: true,
            search: true,
            embedded: false,
        }
    }

    async fn load_video(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        self.base.load_video(source, video_id).await
    }

    async fn load_playlist(
        &self,
        source: &YoutubeAudioSourceManager,
        playlist_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        Client::load_playlist(&self.base, source, playlist_id, selected_video_id).await
    }

    async fn search(
        &self,
        source: &YoutubeAudioSourceManager,
        query: &str,
    ) -> Result<Option<AudioItem>> {
        self.base.search(source, query).await
    }

    async fn get_track_formats(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<TrackFormats> {
        self.base.get_track_formats(source, video_id).await
    }

    async fn load_mix(
        &self,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        self.base.load_mix(source, mix_id, selected_video_id).await
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for WebClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebClient")
    }
}
