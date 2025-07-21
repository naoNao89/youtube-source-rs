use crate::client::config::ClientConfig;
use crate::client::traits::ClientCapabilities;
use crate::{
    AudioItem, Client, ClientOptions, Result, TrackFormats, YoutubeAudioSourceManager, YoutubeError,
};
use async_trait::async_trait;
use serde_json::{Value, json};

/// TV client variants
#[derive(Debug, Clone, PartialEq)]
pub enum TvVariant {
    /// Standard TV client - migrated from Tv.java
    Standard,
    /// TV HTML5 Embedded client - migrated from TvHtml5Embedded.java
    Html5Embedded,
}

/// TV client implementation supporting multiple variants
/// Migrated from Tv.java and TvHtml5Embedded.java
#[derive(Debug, Clone)]
pub struct TvClient {
    options: ClientOptions,
    variant: TvVariant,
}

impl Default for TvClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TvClient {
    pub fn new() -> Self {
        Self {
            options: ClientOptions::default(),
            variant: TvVariant::Standard,
        }
    }

    pub fn with_options(options: ClientOptions) -> Self {
        Self {
            options,
            variant: TvVariant::Standard,
        }
    }

    /// Create TV HTML5 Embedded client variant
    /// Migrated from TvHtml5Embedded.java
    pub fn html5_embedded() -> Self {
        Self {
            options: ClientOptions::default(),
            variant: TvVariant::Html5Embedded,
        }
    }

    /// Create TV HTML5 Embedded client with options
    pub fn html5_embedded_with_options(options: ClientOptions) -> Self {
        Self {
            options,
            variant: TvVariant::Html5Embedded,
        }
    }

    /// Get client configuration based on variant
    fn get_client_config(&self) -> ClientConfig {
        match self.variant {
            TvVariant::Standard => ClientConfig {
                client_name: "TVHTML5".to_string(),
                client_version: "7.20250319.10.00".to_string(),
                user_agent: "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string(),
                ..Default::default()
            },
            TvVariant::Html5Embedded => ClientConfig {
                client_name: "TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string(),
                client_version: "2.0".to_string(),
                user_agent: "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string(),
                third_party_embed_url: Some("https://www.youtube.com".to_string()),
                ..Default::default()
            },
        }
    }
}

#[async_trait]
impl Client for TvClient {
    fn get_identifier(&self) -> &'static str {
        match self.variant {
            TvVariant::Standard => "TVHTML5",
            TvVariant::Html5Embedded => "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
        }
    }

    fn get_options(&self) -> &ClientOptions {
        &self.options
    }

    fn can_handle_request(&self, identifier: &str) -> bool {
        match self.variant {
            TvVariant::Standard => {
                // Standard TV client cannot handle any requests - migrated from Tv.java
                false
            }
            TvVariant::Html5Embedded => {
                // HTML5 Embedded has loose check to avoid loading playlists
                // Migrated from TvHtml5Embedded.java canHandleRequest()
                !identifier.contains("list=") || identifier.contains("list=RD")
            }
        }
    }

    fn get_capabilities(&self) -> ClientCapabilities {
        match self.variant {
            TvVariant::Standard => ClientCapabilities::tv_standard(),
            TvVariant::Html5Embedded => ClientCapabilities::tv_html5_embedded(),
        }
    }

    async fn load_video(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        // Check capabilities before proceeding
        if !self.supports_videos() {
            return Err(YoutubeError::UnsupportedOperation(format!(
                "{} does not support video loading",
                self.get_identifier()
            )));
        }

        match self.variant {
            TvVariant::Standard => {
                // Standard TV client cannot load videos
                Err(YoutubeError::UnsupportedOperation(
                    "TVHTML5 cannot be used to load videos".to_string(),
                ))
            }
            TvVariant::Html5Embedded => {
                // HTML5 Embedded can load videos
                self.load_video_embedded(source, video_id).await
            }
        }
    }

    async fn load_playlist(
        &self,
        _source: &YoutubeAudioSourceManager,
        _playlist_id: &str,
        _selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        // Check capabilities before proceeding
        if !self.supports_playlists() {
            return Err(YoutubeError::UnsupportedOperation(format!(
                "{} does not support playlist loading",
                self.get_identifier()
            )));
        }

        // This should never be reached since no TV variants support playlists
        Err(YoutubeError::UnsupportedOperation(
            "TV clients do not support playlist loading".to_string(),
        ))
    }

    async fn search(
        &self,
        source: &YoutubeAudioSourceManager,
        query: &str,
    ) -> Result<Option<AudioItem>> {
        // Check capabilities before proceeding
        if !self.supports_search() {
            return Err(YoutubeError::UnsupportedOperation(format!(
                "{} does not support search",
                self.get_identifier()
            )));
        }

        let base_client = self.create_base_client(source);
        base_client.search(source, query).await
    }

    async fn load_mix(
        &self,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        match self.variant {
            TvVariant::Standard => {
                // Standard TV client cannot load mixes - migrated from Tv.java
                Err(YoutubeError::UnsupportedOperation(
                    "TVHTML5 cannot be used to load mixes".to_string(),
                ))
            }
            TvVariant::Html5Embedded => {
                // HTML5 Embedded can load mixes
                self.load_mix_embedded(source, mix_id, selected_video_id)
                    .await
            }
        }
    }

    async fn get_track_formats(
        &self,
        _source: &YoutubeAudioSourceManager,
        _video_id: &str,
    ) -> Result<TrackFormats> {
        // TODO: Implement proper track format extraction for TV client
        // For now, return empty formats
        Ok(TrackFormats::new(
            Vec::new(),
            url::Url::parse("https://www.youtube.com/").unwrap(),
        ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TvClient {
    /// Load video for HTML5 Embedded variant
    async fn load_video_embedded(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        let config = self.get_client_config();

        // Create request payload
        let payload = json!({
            "context": {
                "client": {
                    "clientName": config.client_name,
                    "clientVersion": config.client_version,
                    "userAgent": config.user_agent
                },
                "thirdParty": {
                    "embedUrl": config.third_party_embed_url
                }
            },
            "videoId": video_id,
            "playbackContext": {
                "contentPlaybackContext": {
                    "html5Preference": "HTML5_PREF_WANTS"
                }
            }
        });

        // Make API request
        let response = source
            .http_client
            .post("https://youtubei.googleapis.com/youtubei/v1/player")
            .header("Content-Type", "application/json")
            .header("User-Agent", &config.user_agent)
            .json(&payload)
            .send()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to load video: {e}")))?;

        if !response.status().is_success() {
            return Err(YoutubeError::HttpError(format!(
                "Video load failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await.map_err(|e| {
            YoutubeError::ParseError(format!("Failed to parse video response: {e}"))
        })?;

        // Extract video information
        self.extract_video_info(&json, source, video_id).await
    }

    /// Load mix for HTML5 Embedded variant
    async fn load_mix_embedded(
        &self,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        let config = self.get_client_config();

        // Create request payload for mix
        let mut payload = json!({
            "context": {
                "client": {
                    "clientName": config.client_name,
                    "clientVersion": config.client_version,
                    "userAgent": config.user_agent
                },
                "thirdParty": {
                    "embedUrl": config.third_party_embed_url
                }
            },
            "playlistId": mix_id
        });

        // Add selected video if provided
        if let Some(video_id) = selected_video_id {
            payload["videoId"] = json!(video_id);
        }

        // Make API request
        let response = source
            .http_client
            .post("https://youtubei.googleapis.com/youtubei/v1/next")
            .header("Content-Type", "application/json")
            .header("User-Agent", &config.user_agent)
            .json(&payload)
            .send()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to load mix: {e}")))?;

        if !response.status().is_success() {
            return Err(YoutubeError::HttpError(format!(
                "Mix load failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| YoutubeError::ParseError(format!("Failed to parse mix response: {e}")))?;

        // Extract mix information
        self.extract_mix_info(&json, source, mix_id, selected_video_id)
            .await
    }

    /// Create a base client for making Innertube API requests
    fn create_base_client(
        &self,
        _source: &YoutubeAudioSourceManager,
    ) -> crate::client::base::NonMusicClientBase {
        let config = self.get_client_config();
        // Note: We need to extract the actual HTTP client from the source
        // For now, create a new one - this should be improved in the future
        let http_client = crate::http::YoutubeHttpClient::new().unwrap();
        crate::client::base::NonMusicClientBase::new(
            http_client,
            config,
            self.get_identifier().to_string(),
        )
    }

    /// Extract video information from API response
    async fn extract_video_info(
        &self,
        _json: &serde_json::Value,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        // TODO: Implement proper video info extraction for TV client
        // For now, delegate to base client
        let base_client = self.create_base_client(source);
        base_client.load_video(source, video_id).await
    }

    /// Extract mix information from API response
    async fn extract_mix_info(
        &self,
        _json: &serde_json::Value,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        // TODO: Implement proper mix info extraction for TV client
        // For now, delegate to base client
        let base_client = self.create_base_client(source);
        base_client
            .load_mix(source, mix_id, selected_video_id)
            .await
    }
}
