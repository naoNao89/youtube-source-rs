use crate::client::config::ClientConfig;
use crate::client::traits::ClientCapabilities;
use crate::{
    AudioItem, Client, ClientOptions, Result, TrackFormats, YoutubeAudioSourceManager, YoutubeError,
};
use async_trait::async_trait;

/// iOS client implementation
/// Migrated from Ios.java - extends StreamingNonMusicClient
#[derive(Debug, Clone)]
pub struct IosClient {
    options: ClientOptions,
}

impl Default for IosClient {
    fn default() -> Self {
        Self::new()
    }
}

impl IosClient {
    pub fn new() -> Self {
        Self {
            options: ClientOptions::default(),
        }
    }

    pub fn with_options(options: ClientOptions) -> Self {
        Self { options }
    }

    /// Get iOS client configuration
    /// Migrated from Ios.java BASE_CONFIG
    fn get_client_config(&self) -> ClientConfig {
        ClientConfig {
            client_name: "IOS".to_string(),
            client_version: "19.09.3".to_string(),
            user_agent:
                "com.google.ios.youtube/19.09.3 (iPhone14,3; U; CPU iOS 15_6 like Mac OS X)"
                    .to_string(),
            os_name: "iOS".to_string(),
            os_version: "15.6".to_string(),
            device_make: Some("Apple".to_string()),
            device_model: Some("iPhone14,3".to_string()),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Client for IosClient {
    fn get_identifier(&self) -> &'static str {
        "IOS"
    }

    fn get_options(&self) -> &ClientOptions {
        &self.options
    }

    fn can_handle_request(&self, identifier: &str) -> bool {
        // iOS client can handle most requests but has some limitations
        // Cannot load playlists except mixes (like other streaming clients)
        if identifier.contains("list=") && !identifier.contains("list=RD") {
            return false;
        }
        true
    }

    fn get_capabilities(&self) -> ClientCapabilities {
        ClientCapabilities::ios()
    }

    async fn load_video(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        let base_client = self.create_base_client(source);
        base_client.load_video(source, video_id).await
    }

    async fn load_playlist(
        &self,
        source: &YoutubeAudioSourceManager,
        playlist_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        // Check capabilities before proceeding
        if !self.supports_playlists() {
            return Err(YoutubeError::UnsupportedOperation(format!(
                "{} does not support playlist loading",
                self.get_identifier()
            )));
        }

        let base_client = self.create_base_client(source);
        base_client
            .load_playlist(source, playlist_id, selected_video_id)
            .await
    }

    async fn search(
        &self,
        source: &YoutubeAudioSourceManager,
        query: &str,
    ) -> Result<Option<AudioItem>> {
        let base_client = self.create_base_client(source);
        base_client.search(source, query).await
    }

    async fn load_mix(
        &self,
        source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        // Check capabilities before proceeding
        if !self.supports_mixes() {
            return Err(YoutubeError::UnsupportedOperation(format!(
                "{} does not support mix loading",
                self.get_identifier()
            )));
        }

        let base_client = self.create_base_client(source);
        base_client
            .load_mix(source, mix_id, selected_video_id)
            .await
    }

    async fn get_track_formats(
        &self,
        source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<TrackFormats> {
        let base_client = self.create_base_client(source);
        base_client.get_track_formats(source, video_id).await
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl IosClient {
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
}
