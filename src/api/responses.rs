use serde::{Deserialize, Serialize};

/// YouTube Player API response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerResponse {
    #[serde(rename = "videoDetails")]
    pub video_details: Option<VideoDetails>,

    #[serde(rename = "playabilityStatus")]
    pub playability_status: Option<PlayabilityStatus>,

    #[serde(rename = "streamingData")]
    pub streaming_data: Option<StreamingData>,

    #[serde(rename = "playerConfig")]
    pub player_config: Option<PlayerConfig>,
}

/// Video details from player response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,

    pub title: String,

    #[serde(rename = "lengthSeconds")]
    pub length_seconds: Option<String>,

    pub author: Option<String>,

    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,

    #[serde(rename = "isLive")]
    pub is_live: Option<bool>,

    #[serde(rename = "isLiveContent")]
    pub is_live_content: Option<bool>,

    #[serde(rename = "viewCount")]
    pub view_count: Option<String>,

    pub thumbnail: Option<ThumbnailContainer>,
}

/// Playability status
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: Option<String>,

    #[serde(rename = "errorScreen")]
    pub error_screen: Option<serde_json::Value>,
}

/// Streaming data containing formats
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: Option<String>,

    pub formats: Option<Vec<Format>>,

    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Option<Vec<Format>>,
}

/// Individual format information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Format {
    pub itag: u32,

    pub url: Option<String>,

    #[serde(rename = "signatureCipher")]
    pub signature_cipher: Option<String>,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    pub bitrate: Option<u64>,

    #[serde(rename = "contentLength")]
    pub content_length: Option<String>,

    #[serde(rename = "audioChannels")]
    pub audio_channels: Option<u32>,

    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: Option<String>,

    #[serde(rename = "audioTrack")]
    pub audio_track: Option<AudioTrack>,

    #[serde(rename = "isDrc")]
    pub is_drc: Option<bool>,
}

/// Audio track information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioTrack {
    #[serde(rename = "audioIsDefault")]
    pub audio_is_default: Option<bool>,
}

/// Player configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerConfig {
    #[serde(rename = "audioConfig")]
    pub audio_config: Option<serde_json::Value>,
}

/// Thumbnail container
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThumbnailContainer {
    pub thumbnails: Option<Vec<Thumbnail>>,
}

/// Individual thumbnail
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Search response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResponse {
    pub contents: Option<serde_json::Value>,

    #[serde(rename = "estimatedResults")]
    pub estimated_results: Option<String>,
}

/// Browse response (for playlists)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BrowseResponse {
    pub contents: Option<serde_json::Value>,
    pub header: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Generic YouTube API error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

/// Error details
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorDetails {
    pub code: u32,
    pub message: String,
    pub status: Option<String>,
}
