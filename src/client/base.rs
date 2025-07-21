use crate::client::traits::Client;
use crate::config::ClientOptions;
use crate::error::{Result, YoutubeError};
use crate::http::{RequestContext, YoutubeHttpClient};
use crate::playlist::YoutubePlaylist;
use crate::track::{AudioTrackInfo, TrackFormats, YoutubeAudioTrack};
use crate::{AudioItem, YoutubeAudioSourceManager};
use async_trait::async_trait;
use serde_json::Value;

/// Playability status from YouTube API responses
#[derive(Debug, Clone, PartialEq)]
pub enum PlayabilityStatus {
    Ok,
    Error,
    Unplayable,
    LoginRequired,
    LiveStreamOffline,
    Unplayable2,
    ContentCheckRequired,
    Unknown(String),
}

impl From<&str> for PlayabilityStatus {
    fn from(status: &str) -> Self {
        match status {
            "OK" => PlayabilityStatus::Ok,
            "ERROR" => PlayabilityStatus::Error,
            "UNPLAYABLE" => PlayabilityStatus::Unplayable,
            "LOGIN_REQUIRED" => PlayabilityStatus::LoginRequired,
            "LIVE_STREAM_OFFLINE" => PlayabilityStatus::LiveStreamOffline,
            "UNPLAYABLE_2" => PlayabilityStatus::Unplayable2,
            "CONTENT_CHECK_REQUIRED" => PlayabilityStatus::ContentCheckRequired,
            other => PlayabilityStatus::Unknown(other.to_string()),
        }
    }
}

/// Base trait for all non-music YouTube clients
///
/// Based on Java NonMusicClient.java, this provides the foundation for:
/// - Video loading with player API integration
/// - Search functionality with result extraction
/// - Playlist loading with pagination support
/// - Playability status validation and error handling
#[async_trait]
pub trait NonMusicClient: Client {
    /// Load track information from YouTube's Innertube API
    ///
    /// This is the core method that fetches video metadata from the
    /// `/youtubei/v1/player` endpoint and validates playability.
    async fn load_track_info_from_innertube(&self, video_id: &str) -> Result<AudioTrackInfo>;

    /// Load search results from YouTube API
    ///
    /// Uses the `/youtubei/v1/search` endpoint to find videos and playlists
    /// matching the search query.
    async fn load_search_results(&self, query: &str) -> Result<Vec<crate::search::SearchResult>>;

    /// Load playlist information and tracks
    ///
    /// Uses the `/youtubei/v1/browse` endpoint to load playlist metadata
    /// and extract track information with continuation token support.
    async fn load_playlist(&self, playlist_id: &str) -> Result<YoutubePlaylist>;

    /// Get the HTTP client for API requests
    fn get_http_client(&self) -> &YoutubeHttpClient;

    /// Get client configuration for API requests
    fn get_client_config(&self) -> &crate::client::config::ClientConfig;
}

/// Base implementation for NonMusicClient functionality
///
/// This struct provides common functionality that can be shared across
/// different client implementations (Web, Android, iOS, etc.)
#[derive(Debug)]
pub struct NonMusicClientBase {
    http_client: YoutubeHttpClient,
    client_config: crate::client::config::ClientConfig,
    client_name: String,
    options: ClientOptions,
}

impl NonMusicClientBase {
    pub fn new(
        http_client: YoutubeHttpClient,
        client_config: crate::client::config::ClientConfig,
        client_name: String,
    ) -> Self {
        Self {
            http_client,
            client_config,
            client_name,
            options: ClientOptions::default(),
        }
    }

    pub fn with_options(
        http_client: YoutubeHttpClient,
        client_config: crate::client::config::ClientConfig,
        client_name: String,
        options: ClientOptions,
    ) -> Self {
        Self {
            http_client,
            client_config,
            client_name,
            options,
        }
    }

    /// Core implementation of track info loading from Innertube API
    ///
    /// Based on Java NonMusicClient.loadTrackInfoFromInnertube() method.
    /// This method:
    /// 1. Constructs the API request payload
    /// 2. Calls the /youtubei/v1/player endpoint
    /// 3. Parses the JSON response
    /// 4. Validates playability status
    /// 5. Extracts track metadata
    pub async fn load_track_info_from_innertube_impl(
        &self,
        video_id: &str,
    ) -> Result<AudioTrackInfo> {
        // Create request context
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_player_request: true,
            ..Default::default()
        };

        // Build request payload
        let payload = self.build_player_request_payload(video_id)?;

        // Make API request
        let response = self
            .make_innertube_request("player", &payload, context)
            .await?;

        // Parse response and extract track info
        self.parse_track_info_response(video_id, &response).await
    }

    /// Build the request payload for the player API
    fn build_player_request_payload(&self, video_id: &str) -> Result<Value> {
        let mut payload = serde_json::json!({
            "context": self.client_config.to_context_json(),
            "videoId": video_id
        });

        // Add playback context if available
        if let Some(playback_context) = self.client_config.get_playback_context() {
            payload["playbackContext"] = playback_context;
        }

        Ok(payload)
    }

    /// Make a request to YouTube's Innertube API
    async fn make_innertube_request(
        &self,
        endpoint: &str,
        payload: &Value,
        context: RequestContext,
    ) -> Result<Value> {
        let url = format!("https://www.youtube.com/youtubei/v1/{endpoint}");

        // Add API key if available
        let url = if let Some(api_key) = self.client_config.get_api_key() {
            format!("{url}?key={api_key}")
        } else {
            url
        };

        let request = self
            .http_client
            .client()
            .post(&url)
            .header("Content-Type", "application/json")
            .json(payload)
            .build()
            .map_err(|e| YoutubeError::HttpError(format!("Failed to build request: {e}")))?;

        let response = self
            .http_client
            .execute_with_context(request, context)
            .await?;

        if !response.status().is_success() {
            return Err(YoutubeError::ApiError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to parse JSON response: {e}")))?;

        Ok(json)
    }

    /// Parse track information from player API response
    async fn parse_track_info_response(
        &self,
        video_id: &str,
        response: &Value,
    ) -> Result<AudioTrackInfo> {
        // Check playability status
        let playability_status = self.extract_playability_status(response)?;
        if playability_status != PlayabilityStatus::Ok {
            return Err(YoutubeError::VideoUnavailable(format!(
                "Video {video_id} is not playable: {playability_status:?}"
            )));
        }

        // Extract video details
        let video_details = response.get("videoDetails").ok_or_else(|| {
            YoutubeError::ParseError("Missing videoDetails in response".to_string())
        })?;

        let title = video_details
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown Title")
            .to_string();

        let author = video_details
            .get("author")
            .and_then(|a| a.as_str())
            .unwrap_or("Unknown Author")
            .to_string();

        let length_seconds = video_details
            .get("lengthSeconds")
            .and_then(|l| l.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let duration = std::time::Duration::from_secs(length_seconds);

        // Check if it's a live stream
        let is_live = video_details
            .get("isLive")
            .and_then(|l| l.as_bool())
            .unwrap_or(false);

        let uri = format!("https://www.youtube.com/watch?v={video_id}")
            .parse()
            .map_err(YoutubeError::UrlParse)?;

        Ok(AudioTrackInfo {
            title,
            author,
            duration: if is_live {
                std::time::Duration::from_secs(0)
            } else {
                duration
            },
            video_id: video_id.to_string(),
            is_stream: is_live,
            uri,
            thumbnail: None,
            artwork_url: None,
        })
    }

    /// Extract playability status from API response
    fn extract_playability_status(&self, response: &Value) -> Result<PlayabilityStatus> {
        let playability_status = response.get("playabilityStatus").ok_or_else(|| {
            YoutubeError::ParseError("Missing playabilityStatus in response".to_string())
        })?;

        let status_str = playability_status
            .get("status")
            .and_then(|s| s.as_str())
            .ok_or_else(|| {
                YoutubeError::ParseError("Missing status in playabilityStatus".to_string())
            })?;

        Ok(PlayabilityStatus::from(status_str))
    }

    /// Parse track formats from player API response
    async fn parse_track_formats(&self, response: &Value) -> Result<TrackFormats> {
        // Check playability status first
        let playability_status = self.extract_playability_status(response)?;
        if playability_status != PlayabilityStatus::Ok {
            return Err(YoutubeError::VideoUnavailable(format!(
                "Video is not playable: {playability_status:?}"
            )));
        }

        // Extract streaming data
        let streaming_data = response.get("streamingData").ok_or_else(|| {
            YoutubeError::ParseError("Missing streamingData in response".to_string())
        })?;

        let mut formats = Vec::new();

        // Parse adaptive formats (preferred for audio)
        if let Some(adaptive_formats) = streaming_data.get("adaptiveFormats") {
            if let Some(adaptive_array) = adaptive_formats.as_array() {
                for format_data in adaptive_array {
                    if let Ok(format) = self.parse_stream_format(format_data) {
                        // Only include audio formats
                        if format.content_type.starts_with("audio/") {
                            formats.push(format);
                        }
                    }
                }
            }
        }

        // Parse regular formats as fallback
        if let Some(regular_formats) = streaming_data.get("formats") {
            if let Some(regular_array) = regular_formats.as_array() {
                for format_data in regular_array {
                    if let Ok(format) = self.parse_stream_format(format_data) {
                        formats.push(format);
                    }
                }
            }
        }

        if formats.is_empty() {
            return Err(YoutubeError::ParseError(
                "No playable formats found".to_string(),
            ));
        }

        // Extract player script URL for signature decryption
        let player_script_url = self.extract_player_script_url(response)?;

        Ok(TrackFormats::new(formats, player_script_url))
    }

    /// Parse individual stream format from JSON
    fn parse_stream_format(&self, format_data: &Value) -> Result<crate::track::StreamFormat> {
        let itag = format_data
            .get("itag")
            .and_then(|i| i.as_u64())
            .ok_or_else(|| YoutubeError::ParseError("Missing itag in format".to_string()))?
            as u32;

        let content_type = format_data
            .get("mimeType")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown/unknown")
            .to_string();

        let bitrate = format_data
            .get("bitrate")
            .and_then(|b| b.as_u64())
            .unwrap_or(0);

        let content_length = format_data
            .get("contentLength")
            .and_then(|c| c.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let audio_channels = format_data
            .get("audioChannels")
            .and_then(|a| a.as_u64())
            .unwrap_or(2);

        // Extract URL and handle encrypted signatures
        let (url, signature, signature_key) = self.extract_format_url_and_signature(format_data)?;

        let n_parameter = format_data
            .get("n")
            .and_then(|n| n.as_str())
            .map(|n| n.to_string());

        // Determine format info from content type
        let info = self.determine_format_info(&content_type);

        // Check if this is the default audio track
        let is_default_audio_track = format_data
            .get("audioTrack")
            .and_then(|track| track.get("audioIsDefault"))
            .and_then(|default| default.as_bool())
            .unwrap_or(true); // Assume default if not specified

        let is_drc = format_data
            .get("isDrc")
            .and_then(|drc| drc.as_bool())
            .unwrap_or(false);

        Ok(crate::track::StreamFormat {
            info,
            content_type,
            itag,
            bitrate,
            content_length,
            audio_channels,
            url,
            n_parameter,
            signature,
            signature_key,
            is_default_audio_track,
            is_drc,
        })
    }

    /// Extract URL and signature information from format data
    fn extract_format_url_and_signature(
        &self,
        format_data: &Value,
    ) -> Result<(url::Url, Option<String>, Option<String>)> {
        // Check for direct URL first
        if let Some(url_str) = format_data.get("url").and_then(|u| u.as_str()) {
            let url = url_str
                .parse::<url::Url>()
                .map_err(YoutubeError::UrlParse)?;

            // Check for signature parameters in the URL or format data
            let signature = format_data
                .get("s")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());

            let signature_key = format_data
                .get("sp")
                .and_then(|sp| sp.as_str())
                .map(|sp| sp.to_string())
                .or_else(|| Some("sig".to_string())); // Default signature key

            return Ok((url, signature, signature_key));
        }

        // Check for signatureCipher (encrypted format)
        if let Some(cipher_str) = format_data.get("signatureCipher").and_then(|c| c.as_str()) {
            return self.parse_signature_cipher(cipher_str);
        }

        // Fallback: check for individual signature components
        if let Some(signature) = format_data.get("s").and_then(|s| s.as_str()) {
            // We have a signature but no URL - this shouldn't happen in normal cases
            // Create a placeholder URL that will be resolved later
            let placeholder_url = "https://www.youtube.com/placeholder"
                .parse::<url::Url>()
                .map_err(YoutubeError::UrlParse)?;

            let signature_key = format_data
                .get("sp")
                .and_then(|sp| sp.as_str())
                .map(|sp| sp.to_string())
                .or_else(|| Some("sig".to_string()));

            return Ok((placeholder_url, Some(signature.to_string()), signature_key));
        }

        Err(YoutubeError::ParseError(
            "No URL or signature cipher found in format".to_string(),
        ))
    }

    /// Parse signatureCipher parameter into URL and signature components
    fn parse_signature_cipher(
        &self,
        cipher_str: &str,
    ) -> Result<(url::Url, Option<String>, Option<String>)> {
        let mut url_str = None;
        let mut signature = None;
        let mut signature_key = None;

        // Parse URL-encoded parameters
        for pair in cipher_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let decoded_value = urlencoding::decode(value).map_err(|_| {
                    YoutubeError::ParseError("Failed to decode cipher parameter".to_string())
                })?;

                match key {
                    "url" => url_str = Some(decoded_value.to_string()),
                    "s" => signature = Some(decoded_value.to_string()),
                    "sp" => signature_key = Some(decoded_value.to_string()),
                    _ => {} // Ignore other parameters
                }
            }
        }

        let url = url_str
            .ok_or_else(|| {
                YoutubeError::ParseError("No URL found in signature cipher".to_string())
            })?
            .parse::<url::Url>()
            .map_err(YoutubeError::UrlParse)?;

        let signature_key = signature_key.or_else(|| Some("sig".to_string())); // Default signature key

        Ok((url, signature, signature_key))
    }

    /// Determine format info from content type
    fn determine_format_info(&self, content_type: &str) -> Option<crate::track::FormatInfo> {
        use crate::track::FormatInfo;

        if content_type.contains("webm") {
            if content_type.contains("opus") {
                Some(FormatInfo::WebmOpus)
            } else if content_type.contains("vorbis") {
                if content_type.starts_with("video/") {
                    Some(FormatInfo::WebmVideoVorbis)
                } else {
                    Some(FormatInfo::WebmVorbis)
                }
            } else {
                None
            }
        } else if content_type.contains("mp4") {
            if content_type.contains("aac") {
                if content_type.starts_with("video/") {
                    Some(FormatInfo::Mp4VideoAacLc)
                } else {
                    Some(FormatInfo::Mp4AacLc)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extract player script URL for signature decryption
    fn extract_player_script_url(&self, response: &Value) -> Result<url::Url> {
        // Try to find player script URL in various locations
        if let Some(player_config) = response.get("playerConfig") {
            if let Some(assets) = player_config.get("assets") {
                if let Some(js_url) = assets.get("js") {
                    if let Some(js_str) = js_url.as_str() {
                        let full_url = if js_str.starts_with("//") {
                            format!("https:{js_str}")
                        } else if js_str.starts_with("/") {
                            format!("https://www.youtube.com{js_str}")
                        } else {
                            js_str.to_string()
                        };

                        return full_url.parse::<url::Url>().map_err(YoutubeError::UrlParse);
                    }
                }
            }
        }

        // Fallback to a default player script URL
        "https://www.youtube.com/s/player/12345678/player_ias.vflset/en_US/base.js"
            .parse::<url::Url>()
            .map_err(YoutubeError::UrlParse)
    }

    /// Load search results from YouTube API
    async fn load_search_results(&self, query: &str) -> Result<Vec<crate::search::SearchResult>> {
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_search_request: true,
            ..Default::default()
        };

        let payload = self.build_search_request_payload(query)?;
        let response = self
            .make_innertube_request("search", &payload, context)
            .await?;

        // Parse search results from response
        self.parse_search_results(&response).await
    }

    /// Build search request payload
    fn build_search_request_payload(&self, query: &str) -> Result<serde_json::Value> {
        let context = self.client_config.to_context_json();

        Ok(serde_json::json!({
            "context": context,
            "query": query
        }))
    }

    /// Build the request payload for playlist browse API
    fn build_playlist_request_payload(&self, playlist_id: &str) -> Result<Value> {
        let browse_id = if playlist_id.starts_with("VL") {
            playlist_id.to_string()
        } else {
            format!("VL{playlist_id}")
        };

        let payload = serde_json::json!({
            "context": self.client_config.to_context_json(),
            "browseId": browse_id
        });

        Ok(payload)
    }

    /// Build the request payload for playlist continuation
    fn build_playlist_continuation_payload(&self, continuation_token: &str) -> Result<Value> {
        let payload = serde_json::json!({
            "context": self.client_config.to_context_json(),
            "continuation": continuation_token
        });

        Ok(payload)
    }

    /// Build the request payload for mix loading (next API)
    fn build_mix_request_payload(
        &self,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({
            "context": self.client_config.to_context_json(),
            "playlistId": mix_id
        });

        if let Some(video_id) = selected_video_id {
            payload["videoId"] = serde_json::Value::String(video_id.to_string());
        }

        Ok(payload)
    }

    /// Parse search results from API response
    async fn parse_search_results(
        &self,
        response: &serde_json::Value,
    ) -> Result<Vec<crate::search::SearchResult>> {
        let mut results = Vec::new();

        // Navigate to search results
        if let Some(contents) = response.get("contents") {
            if let Some(two_column) = contents.get("twoColumnSearchResultsRenderer") {
                if let Some(primary_contents) = two_column.get("primaryContents") {
                    if let Some(section_list) = primary_contents.get("sectionListRenderer") {
                        if let Some(contents_array) = section_list.get("contents") {
                            if let Some(contents_list) = contents_array.as_array() {
                                for section in contents_list {
                                    if let Some(item_section) = section.get("itemSectionRenderer") {
                                        if let Some(contents) = item_section.get("contents") {
                                            if let Some(items) = contents.as_array() {
                                                for item in items {
                                                    if let Ok(search_result) =
                                                        self.parse_search_item(item)
                                                    {
                                                        results.push(search_result);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Parse playlist response from browse API
    async fn parse_playlist_response(
        &self,
        _playlist_id: &str,
        response: &Value,
    ) -> Result<YoutubePlaylist> {
        // Extract playlist metadata
        let playlist_name = self.extract_playlist_name(response)?;

        // Extract initial video list
        let video_list = self.extract_playlist_video_list(response)?;

        // Extract tracks from video list
        let mut tracks = Vec::new();
        self.extract_playlist_tracks(&video_list, &mut tracks)
            .await?;

        // Check for continuation token
        let mut continuation_token = self.extract_playlist_continuation_token(&video_list);
        let mut page_count = 0;
        const MAX_PAGES: usize = 6; // Limit to prevent excessive API calls

        // Load additional pages if continuation token exists
        while let Some(token) = continuation_token.take() {
            if page_count >= MAX_PAGES {
                break;
            }

            page_count += 1;

            match self.load_playlist_continuation(&token).await {
                Ok((continuation_video_list, next_token)) => {
                    self.extract_playlist_tracks(&continuation_video_list, &mut tracks)
                        .await?;
                    continuation_token = next_token;
                }
                Err(e) => {
                    // Log error but don't fail the entire playlist loading
                    eprintln!("Failed to load playlist continuation: {e}");
                    break;
                }
            }
        }

        if tracks.is_empty() {
            return Err(YoutubeError::ParseError(
                "No tracks found in playlist".to_string(),
            ));
        }

        Ok(YoutubePlaylist::with_tracks(playlist_name, tracks))
    }

    /// Extract playlist name from browse response
    fn extract_playlist_name(&self, response: &Value) -> Result<String> {
        // Try Web client path (metadata.playlistMetadataRenderer)
        if let Some(name) = response
            .get("metadata")
            .and_then(|m| m.get("playlistMetadataRenderer"))
            .and_then(|p| p.get("title"))
            .and_then(|t| t.as_str())
        {
            return Ok(name.to_string());
        }

        // Try NonMusicClient path (header.playlistHeaderRenderer)
        if let Some(name) = response
            .get("header")
            .and_then(|h| h.get("playlistHeaderRenderer"))
            .and_then(|p| p.get("title"))
            .and_then(|t| t.get("runs"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
        {
            return Ok(name.to_string());
        }

        // Try iOS/MWeb client path (header.pageHeaderRenderer)
        if let Some(name) = response
            .get("header")
            .and_then(|h| h.get("pageHeaderRenderer"))
            .and_then(|p| p.get("pageTitle"))
            .and_then(|t| t.as_str())
        {
            return Ok(name.to_string());
        }

        // Try alternative title path
        if let Some(name) = response
            .get("header")
            .and_then(|h| h.get("playlistHeaderRenderer"))
            .and_then(|p| p.get("title"))
            .and_then(|t| t.as_str())
        {
            return Ok(name.to_string());
        }

        // Fallback to a default name
        Ok("YouTube Playlist".to_string())
    }

    /// Extract playlist video list from browse response
    fn extract_playlist_video_list(&self, response: &Value) -> Result<Value> {
        // Try Web client path (twoColumnBrowseResultsRenderer)
        if let Some(video_list) = response
            .get("contents")
            .and_then(|c| c.get("twoColumnBrowseResultsRenderer"))
            .and_then(|t| t.get("tabs"))
            .and_then(|tabs| tabs.get(0))
            .and_then(|tab| tab.get("tabRenderer"))
            .and_then(|tr| tr.get("content"))
            .and_then(|content| content.get("sectionListRenderer"))
            .and_then(|slr| slr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|item| item.get("itemSectionRenderer"))
            .and_then(|isr| isr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|playlist| playlist.get("playlistVideoListRenderer"))
        {
            return Ok(video_list.clone());
        }

        // Try NonMusicClient path (singleColumnBrowseResultsRenderer)
        if let Some(video_list) = response
            .get("contents")
            .and_then(|c| c.get("singleColumnBrowseResultsRenderer"))
            .and_then(|t| t.get("tabs"))
            .and_then(|tabs| tabs.get(0))
            .and_then(|tab| tab.get("tabRenderer"))
            .and_then(|tr| tr.get("content"))
            .and_then(|content| content.get("sectionListRenderer"))
            .and_then(|slr| slr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|playlist| playlist.get("playlistVideoListRenderer"))
        {
            return Ok(video_list.clone());
        }

        // Try iOS client path (singleColumnBrowseResultsRenderer with itemSectionRenderer)
        if let Some(video_list) = response
            .get("contents")
            .and_then(|c| c.get("singleColumnBrowseResultsRenderer"))
            .and_then(|t| t.get("tabs"))
            .and_then(|tabs| tabs.get(0))
            .and_then(|tab| tab.get("tabRenderer"))
            .and_then(|tr| tr.get("content"))
            .and_then(|content| content.get("sectionListRenderer"))
            .and_then(|slr| slr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|item| item.get("itemSectionRenderer"))
            .and_then(|isr| isr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|playlist| playlist.get("playlistVideoListRenderer"))
        {
            return Ok(video_list.clone());
        }

        // Try TvHtml5Embedded path (direct sectionListRenderer)
        if let Some(video_list) = response
            .get("contents")
            .and_then(|c| c.get("sectionListRenderer"))
            .and_then(|slr| slr.get("contents"))
            .and_then(|contents| contents.get(0))
            .and_then(|playlist| playlist.get("playlistVideoListRenderer"))
        {
            return Ok(video_list.clone());
        }

        Err(YoutubeError::ParseError(
            "Could not find playlist video list".to_string(),
        ))
    }

    /// Extract tracks from playlist video list
    async fn extract_playlist_tracks(
        &self,
        video_list: &Value,
        tracks: &mut Vec<crate::track::YoutubeAudioTrack>,
    ) -> Result<()> {
        let contents = video_list.get("contents").unwrap_or(video_list);

        if let Some(videos) = contents.as_array() {
            for video in videos {
                if let Some(track) = self.extract_playlist_track(video).await? {
                    tracks.push(track);
                }
            }
        }

        Ok(())
    }

    /// Extract a single track from playlist video renderer
    async fn extract_playlist_track(
        &self,
        video: &Value,
    ) -> Result<Option<crate::track::YoutubeAudioTrack>> {
        let renderer = video.get("playlistVideoRenderer");
        if renderer.is_none() {
            return Ok(None);
        }

        let renderer = renderer.unwrap();

        // Extract video ID
        let video_id = renderer
            .get("videoId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| YoutubeError::ParseError("No video ID in playlist item".to_string()))?;

        // Extract title
        let title = renderer
            .get("title")
            .and_then(|t| t.get("runs"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .or_else(|| {
                renderer
                    .get("title")
                    .and_then(|t| t.get("simpleText"))
                    .and_then(|t| t.as_str())
            })
            .unwrap_or("Unknown Title");

        // Extract author
        let author = renderer
            .get("shortBylineText")
            .and_then(|s| s.get("runs"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown Artist");

        // Extract duration
        let duration_text = renderer
            .get("lengthText")
            .and_then(|l| l.get("simpleText"))
            .and_then(|t| t.as_str())
            .unwrap_or("0:00");

        let duration = self.parse_duration_text(duration_text);

        // Create track info
        let track_info = AudioTrackInfo {
            title: title.to_string(),
            author: author.to_string(),
            duration,
            video_id: video_id.to_string(),
            is_stream: false,
            uri: format!("https://www.youtube.com/watch?v={video_id}")
                .parse()
                .map_err(YoutubeError::UrlParse)?,
            thumbnail: None,
            artwork_url: None,
        };

        // Create track with source manager reference
        let track = crate::track::YoutubeAudioTrack {
            info: track_info,
            source_manager: std::sync::Arc::new(crate::YoutubeAudioSourceManager::new()),
        };

        Ok(Some(track))
    }

    /// Extract continuation token from playlist video list
    fn extract_playlist_continuation_token(&self, video_list: &Value) -> Option<String> {
        // Try different paths for continuation token
        video_list
            .get("continuations")
            .and_then(|c| c.get(0))
            .and_then(|cont| cont.get("nextContinuationData"))
            .and_then(|ncd| ncd.get("continuation"))
            .and_then(|token| token.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                // Alternative path
                video_list
                    .get("contents")
                    .and_then(|contents| {
                        if let Some(array) = contents.as_array() {
                            array.last()
                        } else {
                            None
                        }
                    })
                    .and_then(|last| last.get("continuationItemRenderer"))
                    .and_then(|cir| cir.get("continuationEndpoint"))
                    .and_then(|ce| ce.get("continuationCommand"))
                    .and_then(|cc| cc.get("token"))
                    .and_then(|token| token.as_str())
                    .map(|s| s.to_string())
            })
    }

    /// Load playlist continuation page
    async fn load_playlist_continuation(
        &self,
        continuation_token: &str,
    ) -> Result<(Value, Option<String>)> {
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_browse_request: true,
            ..Default::default()
        };

        let payload = self.build_playlist_continuation_payload(continuation_token)?;
        let response = self
            .make_innertube_request("browse", &payload, context)
            .await?;

        // Extract continuation video list
        let video_list = self.extract_playlist_continuation_videos(&response)?;

        // Extract next continuation token
        let next_token = self.extract_playlist_continuation_token(&video_list);

        Ok((video_list, next_token))
    }

    /// Extract playlist continuation videos from response
    fn extract_playlist_continuation_videos(&self, response: &Value) -> Result<Value> {
        if let Some(video_list) = response
            .get("onResponseReceivedActions")
            .and_then(|actions| actions.get(0))
            .and_then(|action| action.get("appendContinuationItemsAction"))
            .and_then(|acia| acia.get("continuationItems"))
        {
            return Ok(video_list.clone());
        }

        Err(YoutubeError::ParseError(
            "Could not find continuation videos".to_string(),
        ))
    }

    /// Parse duration text (e.g., "3:45" or "1:23:45") to Duration
    fn parse_duration_text(&self, duration_text: &str) -> std::time::Duration {
        let parts: Vec<&str> = duration_text.split(':').collect();
        let mut total_seconds = 0u64;

        match parts.len() {
            1 => {
                // Just seconds
                if let Ok(seconds) = parts[0].parse::<u64>() {
                    total_seconds = seconds;
                }
            }
            2 => {
                // Minutes:Seconds
                if let (Ok(minutes), Ok(seconds)) =
                    (parts[0].parse::<u64>(), parts[1].parse::<u64>())
                {
                    total_seconds = minutes * 60 + seconds;
                }
            }
            3 => {
                // Hours:Minutes:Seconds
                if let (Ok(hours), Ok(minutes), Ok(seconds)) = (
                    parts[0].parse::<u64>(),
                    parts[1].parse::<u64>(),
                    parts[2].parse::<u64>(),
                ) {
                    total_seconds = hours * 3600 + minutes * 60 + seconds;
                }
            }
            _ => {
                // Invalid format, default to 0
            }
        }

        std::time::Duration::from_secs(total_seconds)
    }

    /// Parse mix response from next API
    async fn parse_mix_response(&self, _mix_id: &str, response: &Value) -> Result<YoutubePlaylist> {
        // Extract mix playlist data
        let playlist_data = self.extract_mix_playlist_data(response)?;

        // Extract title
        let title = playlist_data
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("YouTube Mix")
            .to_string();

        // Extract tracks from contents
        let mut tracks = Vec::new();
        if let Some(contents) = playlist_data.get("contents").and_then(|c| c.as_array()) {
            for item in contents {
                if let Some(track) = self.extract_mix_track(item).await? {
                    tracks.push(track);
                }
            }
        }

        if tracks.is_empty() {
            return Err(YoutubeError::ParseError(
                "No tracks found in mix".to_string(),
            ));
        }

        let mut playlist = YoutubePlaylist::with_tracks(title, tracks);
        playlist.is_search_result = false; // Mixes are not search results

        Ok(playlist)
    }

    /// Extract mix playlist data from next response
    fn extract_mix_playlist_data(&self, response: &Value) -> Result<Value> {
        // Try different paths for mix playlist data
        if let Some(playlist_data) = response
            .get("contents")
            .and_then(|c| c.get("singleColumnWatchNextResults"))
            .and_then(|scwnr| scwnr.get("playlist"))
            .and_then(|p| p.get("playlist"))
        {
            return Ok(playlist_data.clone());
        }

        // Alternative path for different client types
        if let Some(playlist_data) = response
            .get("contents")
            .and_then(|c| c.get("twoColumnWatchNextResults"))
            .and_then(|tcwnr| tcwnr.get("playlist"))
            .and_then(|p| p.get("playlist"))
        {
            return Ok(playlist_data.clone());
        }

        Err(YoutubeError::ParseError(
            "Could not find mix playlist data".to_string(),
        ))
    }

    /// Extract a single track from mix playlist panel video renderer
    async fn extract_mix_track(
        &self,
        item: &Value,
    ) -> Result<Option<crate::track::YoutubeAudioTrack>> {
        let renderer = item.get("playlistPanelVideoRenderer");
        if renderer.is_none() {
            return Ok(None);
        }

        let renderer = renderer.unwrap();

        // Extract video ID
        let video_id = renderer
            .get("videoId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| YoutubeError::ParseError("No video ID in mix item".to_string()))?;

        // Extract title
        let title = renderer
            .get("title")
            .and_then(|t| t.get("runs"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .or_else(|| {
                renderer
                    .get("title")
                    .and_then(|t| t.get("simpleText"))
                    .and_then(|t| t.as_str())
            })
            .unwrap_or("Unknown Title");

        // Extract author
        let author = renderer
            .get("shortBylineText")
            .and_then(|s| s.get("runs"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .or_else(|| {
                renderer
                    .get("longBylineText")
                    .and_then(|l| l.get("runs"))
                    .and_then(|r| r.get(0))
                    .and_then(|r| r.get("text"))
                    .and_then(|t| t.as_str())
            })
            .unwrap_or("Unknown Artist");

        // Extract duration
        let duration_text = renderer
            .get("lengthText")
            .and_then(|l| l.get("simpleText"))
            .and_then(|t| t.as_str())
            .unwrap_or("0:00");

        let duration = self.parse_duration_text(duration_text);

        // Create track info
        let track_info = AudioTrackInfo {
            title: title.to_string(),
            author: author.to_string(),
            duration,
            video_id: video_id.to_string(),
            is_stream: false,
            uri: format!("https://www.youtube.com/watch?v={video_id}")
                .parse()
                .map_err(YoutubeError::UrlParse)?,
            thumbnail: None,
            artwork_url: None,
        };

        // Create track with source manager reference
        let track = crate::track::YoutubeAudioTrack {
            info: track_info,
            source_manager: std::sync::Arc::new(crate::YoutubeAudioSourceManager::new()),
        };

        Ok(Some(track))
    }

    /// Parse individual search result item
    fn parse_search_item(&self, item: &serde_json::Value) -> Result<crate::search::SearchResult> {
        // Try to parse as video renderer
        if let Some(video_renderer) = item.get("videoRenderer") {
            return self.parse_video_search_result(video_renderer);
        }

        // Try to parse as playlist renderer
        if let Some(playlist_renderer) = item.get("playlistRenderer") {
            return self.parse_playlist_search_result(playlist_renderer);
        }

        // Try to parse as channel renderer
        if let Some(channel_renderer) = item.get("channelRenderer") {
            return self.parse_channel_search_result(channel_renderer);
        }

        Err(YoutubeError::ParseError(
            "Unknown search result type".to_string(),
        ))
    }

    /// Parse video search result
    fn parse_video_search_result(
        &self,
        video_renderer: &serde_json::Value,
    ) -> Result<crate::search::SearchResult> {
        let video_id = video_renderer
            .get("videoId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                YoutubeError::ParseError("Missing video ID in search result".to_string())
            })?
            .to_string();

        let title = video_renderer
            .get("title")
            .and_then(|t| t.get("runs"))
            .and_then(|runs| runs.as_array())
            .and_then(|arr| arr.first())
            .and_then(|first| first.get("text"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown Title")
            .to_string();

        let author = video_renderer
            .get("ownerText")
            .and_then(|owner| owner.get("runs"))
            .and_then(|runs| runs.as_array())
            .and_then(|arr| arr.first())
            .and_then(|first| first.get("text"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown Author")
            .to_string();

        // Parse duration
        let duration_text = video_renderer
            .get("lengthText")
            .and_then(|length| length.get("simpleText"))
            .and_then(|text| text.as_str())
            .unwrap_or("0:00");

        let duration = self.parse_duration_text(duration_text);

        let uri = format!("https://www.youtube.com/watch?v={video_id}");

        Ok(crate::search::SearchResult::Video {
            video_id,
            title,
            author,
            duration,
            uri,
        })
    }

    /// Parse playlist search result
    fn parse_playlist_search_result(
        &self,
        playlist_renderer: &serde_json::Value,
    ) -> Result<crate::search::SearchResult> {
        let playlist_id = playlist_renderer
            .get("playlistId")
            .and_then(|p| p.as_str())
            .ok_or_else(|| {
                YoutubeError::ParseError("Missing playlist ID in search result".to_string())
            })?
            .to_string();

        let title = playlist_renderer
            .get("title")
            .and_then(|t| t.get("simpleText"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown Playlist")
            .to_string();

        let author = playlist_renderer
            .get("shortBylineText")
            .and_then(|byline| byline.get("runs"))
            .and_then(|runs| runs.as_array())
            .and_then(|arr| arr.first())
            .and_then(|first| first.get("text"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown Author")
            .to_string();

        let video_count = playlist_renderer
            .get("videoCount")
            .and_then(|count| count.as_str())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        let uri = format!("https://www.youtube.com/playlist?list={playlist_id}");

        Ok(crate::search::SearchResult::Playlist {
            playlist_id,
            title,
            author,
            video_count,
            uri,
        })
    }

    /// Parse channel search result
    fn parse_channel_search_result(
        &self,
        channel_renderer: &serde_json::Value,
    ) -> Result<crate::search::SearchResult> {
        let channel_id = channel_renderer
            .get("channelId")
            .and_then(|c| c.as_str())
            .ok_or_else(|| {
                YoutubeError::ParseError("Missing channel ID in search result".to_string())
            })?
            .to_string();

        let title = channel_renderer
            .get("title")
            .and_then(|t| t.get("simpleText"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown Channel")
            .to_string();

        let subscriber_count = channel_renderer
            .get("subscriberCountText")
            .and_then(|count| count.get("simpleText"))
            .and_then(|text| text.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let uri = format!("https://www.youtube.com/channel/{channel_id}");

        Ok(crate::search::SearchResult::Channel {
            channel_id,
            title,
            subscriber_count,
            uri,
        })
    }
}

#[async_trait]
impl NonMusicClient for NonMusicClientBase {
    async fn load_track_info_from_innertube(&self, video_id: &str) -> Result<AudioTrackInfo> {
        self.load_track_info_from_innertube_impl(video_id).await
    }

    async fn load_search_results(&self, query: &str) -> Result<Vec<crate::search::SearchResult>> {
        // Delegate to the actual implementation
        self.load_search_results(query).await
    }

    async fn load_playlist(&self, playlist_id: &str) -> Result<YoutubePlaylist> {
        // Load playlist using browse API
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_browse_request: true,
            ..Default::default()
        };

        let payload = self.build_playlist_request_payload(playlist_id)?;
        let response = self
            .make_innertube_request("browse", &payload, context)
            .await?;

        // Parse playlist from response
        self.parse_playlist_response(playlist_id, &response).await
    }

    fn get_http_client(&self) -> &YoutubeHttpClient {
        &self.http_client
    }

    fn get_client_config(&self) -> &crate::client::config::ClientConfig {
        &self.client_config
    }
}

#[async_trait]
impl Client for NonMusicClientBase {
    fn get_identifier(&self) -> &'static str {
        // This will be overridden by specific clients
        "BASE"
    }

    fn get_options(&self) -> &ClientOptions {
        &self.options
    }

    fn can_handle_request(&self, _identifier: &str) -> bool {
        // Base implementation accepts all requests
        // Specific clients can override this
        true
    }

    fn get_capabilities(&self) -> crate::client::traits::ClientCapabilities {
        // Base implementation supports all capabilities
        // Specific clients should override this
        crate::client::traits::ClientCapabilities {
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
        _source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<Option<AudioItem>> {
        let track_info = self.load_track_info_from_innertube(video_id).await?;

        // Create basic track
        let track = YoutubeAudioTrack {
            info: track_info,
            source_manager: std::sync::Arc::new(_source.clone()),
        };

        Ok(Some(AudioItem::Track(track)))
    }

    async fn load_playlist(
        &self,
        _source: &YoutubeAudioSourceManager,
        playlist_id: &str,
        _selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        let playlist = NonMusicClient::load_playlist(self, playlist_id).await?;
        Ok(Some(AudioItem::Playlist(playlist)))
    }

    async fn search(
        &self,
        _source: &YoutubeAudioSourceManager,
        query: &str,
    ) -> Result<Option<AudioItem>> {
        let results = self.load_search_results(query).await?;

        if results.is_empty() {
            Ok(None)
        } else {
            // Convert SearchResults to YoutubeSearchResult
            let mut youtube_search_result =
                crate::search::YoutubeSearchResult::new(query.to_string());

            for result in results {
                match result {
                    crate::search::SearchResult::Video {
                        video_id,
                        title,
                        author,
                        duration,
                        uri,
                    } => {
                        // Create AudioTrackInfo from search result
                        let track_info = AudioTrackInfo {
                            title,
                            author,
                            duration,
                            video_id,
                            uri: uri
                                .parse()
                                .unwrap_or_else(|_| "https://www.youtube.com/".parse().unwrap()),
                            is_stream: false,
                            thumbnail: None,
                            artwork_url: None,
                        };

                        // Create YoutubeAudioTrack
                        let track = YoutubeAudioTrack {
                            info: track_info,
                            source_manager: std::sync::Arc::new(_source.clone()),
                        };

                        youtube_search_result.add_track(track);
                    }
                    crate::search::SearchResult::Playlist {
                        playlist_id: _,
                        title,
                        author: _,
                        video_count: _,
                        uri: _,
                    } => {
                        // Create basic playlist info
                        let playlist = YoutubePlaylist {
                            name: title,
                            selected_track: None,
                            tracks: Vec::new(), // Will be loaded later
                            is_search_result: true,
                        };

                        youtube_search_result.add_playlist(playlist);
                    }
                    crate::search::SearchResult::Channel { .. } => {
                        // Skip channels for now as they don't fit into the current structure
                        continue;
                    }
                }
            }

            Ok(Some(AudioItem::SearchResult(youtube_search_result)))
        }
    }

    async fn get_track_formats(
        &self,
        _source: &YoutubeAudioSourceManager,
        video_id: &str,
    ) -> Result<TrackFormats> {
        // Load player response to get streaming data
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_player_request: true,
            ..Default::default()
        };

        let payload = self.build_player_request_payload(video_id)?;
        let response = self
            .make_innertube_request("player", &payload, context)
            .await?;

        // Parse streaming data from response
        self.parse_track_formats(&response).await
    }

    async fn load_mix(
        &self,
        _source: &YoutubeAudioSourceManager,
        mix_id: &str,
        selected_video_id: Option<&str>,
    ) -> Result<Option<AudioItem>> {
        // Load mix using next API
        let context = RequestContext {
            client_name: Some(self.client_name.clone()),
            is_next_request: true,
            ..Default::default()
        };

        let payload = self.build_mix_request_payload(mix_id, selected_video_id)?;
        let response = self
            .make_innertube_request("next", &payload, context)
            .await?;

        // Parse mix from response
        match self.parse_mix_response(mix_id, &response).await {
            Ok(playlist) => Ok(Some(AudioItem::Playlist(playlist))),
            Err(e) => {
                eprintln!("Failed to parse mix response: {e}");
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
