use super::{endpoints::*, responses::*};
use crate::error::{Result, YoutubeError};
use reqwest::Client as HttpClient;
use serde_json::{Value, json};

/// YouTube API client for making InnerTube requests
#[derive(Debug, Clone)]
pub struct YoutubeApiClient {
    http_client: HttpClient,
    api_key: String,
    visitor_data: Option<String>,
}

impl YoutubeApiClient {
    /// Create a new YouTube API client
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: ClientConstants::DEFAULT_API_KEY.to_string(),
            visitor_data: None,
        }
    }

    /// Create a new YouTube API client with custom API key
    pub fn with_api_key(api_key: String) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key,
            visitor_data: None,
        }
    }

    /// Set visitor data for requests
    pub fn set_visitor_data(&mut self, visitor_data: Option<String>) {
        self.visitor_data = visitor_data;
    }

    /// Get player information for a video
    pub async fn get_player_info(
        &self,
        video_id: &str,
        client_name: &str,
        client_version: &str,
        user_agent: &str,
    ) -> Result<PlayerResponse> {
        let url = YoutubeEndpoints::get_player_url(&self.api_key);

        let context = self.build_context(client_name, client_version);
        let body = json!({
            "context": context,
            "videoId": video_id,
            "playbackContext": {
                "contentPlaybackContext": {
                    "html5Preference": "HTML5_PREF_WANTS"
                }
            }
        });

        let request = self
            .http_client
            .post(&url)
            .header("User-Agent", user_agent)
            .header("Content-Type", "application/json")
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com/")
            .json(&body);

        let response = request
            .send()
            .await
            .map_err(|e| YoutubeError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(YoutubeError::ApiError(format!("HTTP {status}: {text}")));
        }

        let player_response: PlayerResponse = response
            .json()
            .await
            .map_err(|e| YoutubeError::ParseError(e.to_string()))?;

        Ok(player_response)
    }

    /// Search for videos
    pub async fn search(
        &self,
        query: &str,
        client_name: &str,
        client_version: &str,
        user_agent: &str,
    ) -> Result<SearchResponse> {
        let url = YoutubeEndpoints::get_search_url(&self.api_key);

        let context = self.build_context(client_name, client_version);
        let body = json!({
            "context": context,
            "query": query
        });

        let request = self
            .http_client
            .post(&url)
            .header("User-Agent", user_agent)
            .header("Content-Type", "application/json")
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com/")
            .json(&body);

        let response = request
            .send()
            .await
            .map_err(|e| YoutubeError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(YoutubeError::ApiError(format!("HTTP {status}: {text}")));
        }

        let search_response: SearchResponse = response
            .json()
            .await
            .map_err(|e| YoutubeError::ParseError(e.to_string()))?;

        Ok(search_response)
    }

    /// Get playlist information
    pub async fn get_playlist(
        &self,
        playlist_id: &str,
        client_name: &str,
        client_version: &str,
        user_agent: &str,
    ) -> Result<BrowseResponse> {
        let url = YoutubeEndpoints::get_browse_url(&self.api_key);

        let context = self.build_context(client_name, client_version);
        let body = json!({
            "context": context,
            "browseId": format!("VL{}", playlist_id)
        });

        let request = self
            .http_client
            .post(&url)
            .header("User-Agent", user_agent)
            .header("Content-Type", "application/json")
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com/")
            .json(&body);

        let response = request
            .send()
            .await
            .map_err(|e| YoutubeError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(YoutubeError::ApiError(format!("HTTP {status}: {text}")));
        }

        let browse_response: BrowseResponse = response
            .json()
            .await
            .map_err(|e| YoutubeError::ParseError(e.to_string()))?;

        Ok(browse_response)
    }

    /// Build context object for API requests
    fn build_context(&self, client_name: &str, client_version: &str) -> Value {
        let mut client = json!({
            "clientName": client_name,
            "clientVersion": client_version
        });

        if let Some(visitor_data) = &self.visitor_data {
            client["visitorData"] = json!(visitor_data);
        }

        json!({
            "client": client
        })
    }
}

impl Default for YoutubeApiClient {
    fn default() -> Self {
        Self::new()
    }
}
