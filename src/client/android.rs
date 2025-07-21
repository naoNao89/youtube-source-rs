use crate::client::config::ClientConfig;
use crate::client::traits::ClientCapabilities;
use crate::{
    AudioItem, Client, ClientOptions, Result, TrackFormats, YoutubeAudioSourceManager, YoutubeError,
};
use async_trait::async_trait;

/// Android client variants
#[derive(Debug, Clone, PartialEq)]
pub enum AndroidVariant {
    /// Standard Android client
    Standard,
    /// Android Music client - migrated from AndroidMusic.java
    Music,
    /// Android VR client - migrated from AndroidVr.java
    Vr,
}

/// Android client implementation supporting multiple variants
/// Migrated from Android.java, AndroidMusic.java, and AndroidVr.java
#[derive(Debug, Clone)]
pub struct AndroidClient {
    options: ClientOptions,
    variant: AndroidVariant,
}

impl Default for AndroidClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidClient {
    pub fn new() -> Self {
        Self {
            options: ClientOptions::default(),
            variant: AndroidVariant::Standard,
        }
    }

    pub fn with_options(options: ClientOptions) -> Self {
        Self {
            options,
            variant: AndroidVariant::Standard,
        }
    }

    /// Create Android Music client variant
    /// Migrated from AndroidMusic.java
    pub fn music() -> Self {
        Self {
            options: ClientOptions::default(),
            variant: AndroidVariant::Music,
        }
    }

    /// Create Android Music client with options
    pub fn music_with_options(options: ClientOptions) -> Self {
        Self {
            options,
            variant: AndroidVariant::Music,
        }
    }

    /// Create Android VR client variant
    /// Migrated from AndroidVr.java
    pub fn vr() -> Self {
        Self {
            options: ClientOptions::default(),
            variant: AndroidVariant::Vr,
        }
    }

    /// Create Android VR client with options
    pub fn vr_with_options(options: ClientOptions) -> Self {
        Self {
            options,
            variant: AndroidVariant::Vr,
        }
    }

    /// Get client configuration based on variant
    fn get_client_config(&self) -> ClientConfig {
        match self.variant {
            AndroidVariant::Standard => ClientConfig {
                client_name: "ANDROID".to_string(),
                client_version: "19.09.37".to_string(),
                user_agent: "com.google.android.youtube/19.09.37 (Linux; U; Android 11) gzip".to_string(),
                os_name: "Android".to_string(),
                os_version: "11".to_string(),
                android_sdk_version: Some(30),
                ..Default::default()
            },
            AndroidVariant::Music => ClientConfig {
                client_name: "ANDROID_MUSIC".to_string(),
                client_version: "7.11.50".to_string(),
                user_agent: "com.google.android.apps.youtube.music/7.11.50 (Linux; U; Android 11) gzip".to_string(),
                os_name: "Android".to_string(),
                os_version: "11".to_string(),
                android_sdk_version: Some(30),
                ..Default::default()
            },
            AndroidVariant::Vr => ClientConfig {
                client_name: "ANDROID_VR".to_string(),
                client_version: "1.60.19".to_string(),
                user_agent: "com.google.android.apps.youtube.vr.oculus/1.60.19 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip".to_string(),
                os_name: "Android".to_string(),
                os_version: "12L".to_string(),
                android_sdk_version: Some(32),
                ..Default::default()
            },
        }
    }
}

#[async_trait]
impl Client for AndroidClient {
    fn get_identifier(&self) -> &'static str {
        match self.variant {
            AndroidVariant::Standard => "ANDROID",
            AndroidVariant::Music => "ANDROID_MUSIC",
            AndroidVariant::Vr => "ANDROID_VR",
        }
    }

    fn get_options(&self) -> &ClientOptions {
        &self.options
    }

    fn can_handle_request(&self, identifier: &str) -> bool {
        match self.variant {
            AndroidVariant::Music => {
                // AndroidMusic has restrictions - migrated from AndroidMusic.java
                // Cannot load playlists except mixes
                if identifier.contains("list=") && !identifier.contains("list=RD") {
                    return false;
                }
                true
            }
            AndroidVariant::Standard | AndroidVariant::Vr => {
                // Standard Android and VR can handle most requests
                true
            }
        }
    }

    fn get_capabilities(&self) -> ClientCapabilities {
        match self.variant {
            AndroidVariant::Standard => ClientCapabilities::android_standard(),
            AndroidVariant::Music => ClientCapabilities::android_music(),
            AndroidVariant::Vr => ClientCapabilities::android_vr(),
        }
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

impl AndroidClient {
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
