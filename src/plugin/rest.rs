use serde::{Deserialize, Serialize};


/// REST handler for YouTube plugin endpoints
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/YoutubeRestHandler.java`
/// 
/// This provides REST API endpoints for configuring the YouTube source plugin at runtime.
pub struct YoutubeRestHandler {
    // TODO: Add reference to the plugin loader or source manager
}

impl YoutubeRestHandler {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Handle POST /youtube endpoint for configuration updates
    pub async fn handle_post_config(&self, request: ConfigUpdateRequest) -> Result<ConfigUpdateResponse, RestError> {
        // TODO: Implement configuration update logic
        // This would update OAuth tokens, PoToken, etc.
        
        if let Some(_refresh_token) = request.refresh_token {
            // Update OAuth refresh token
            // source.update_oauth_token(refresh_token).await?;
        }
        
        if let Some(po_token) = request.po_token {
            if let Some(visitor_data) = request.visitor_data {
                // Update PoToken and visitor data
                crate::YoutubeSource::set_po_token_and_visitor_data(
                    Some(po_token),
                    Some(visitor_data)
                );
            }
        }
        
        Ok(ConfigUpdateResponse {
            success: true,
            message: "Configuration updated successfully".to_string(),
        })
    }
    
    /// Handle GET /youtube endpoint for current configuration
    pub async fn handle_get_config(&self) -> Result<ConfigResponse, RestError> {
        // TODO: Get current configuration from source manager
        Ok(ConfigResponse {
            refresh_token: None, // Don't expose the actual token for security
            has_refresh_token: false, // TODO: Check if token exists
            po_token_configured: false, // TODO: Check if PoToken is configured
        })
    }
    
    /// Handle GET /youtube/stream/{videoId} endpoint for direct streaming
    pub async fn handle_stream_request(
        &self, 
        _video_id: &str,
        _params: StreamRequestParams
    ) -> Result<StreamResponse, RestError> {
        // TODO: Implement direct streaming endpoint
        // This would return the stream URL or proxy the stream directly
        
        Err(RestError::NotImplemented("Direct streaming not yet implemented".to_string()))
    }
    
    /// Handle GET /youtube/oauth/{refreshToken} endpoint for OAuth token refresh
    pub async fn handle_oauth_refresh(&self, _refresh_token: &str) -> Result<OAuthRefreshResponse, RestError> {
        // TODO: Implement OAuth token refresh
        // This would use the refresh token to get a new access token
        
        Err(RestError::NotImplemented("OAuth refresh not yet implemented".to_string()))
    }
}

impl Default for YoutubeRestHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Request body for configuration updates
#[derive(Debug, Deserialize)]
pub struct ConfigUpdateRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    
    #[serde(rename = "skipInitialization")]
    pub skip_initialization: Option<bool>,
    
    #[serde(rename = "poToken")]
    pub po_token: Option<String>,
    
    #[serde(rename = "visitorData")]
    pub visitor_data: Option<String>,
}

/// Response for configuration updates
#[derive(Debug, Serialize)]
pub struct ConfigUpdateResponse {
    pub success: bool,
    pub message: String,
}

/// Response for configuration queries
#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    
    #[serde(rename = "hasRefreshToken")]
    pub has_refresh_token: bool,
    
    #[serde(rename = "poTokenConfigured")]
    pub po_token_configured: bool,
}

/// Parameters for stream requests
#[derive(Debug, Deserialize)]
pub struct StreamRequestParams {
    pub itag: Option<u32>,
    
    #[serde(rename = "withClient")]
    pub with_client: Option<String>,
}

/// Response for stream requests
#[derive(Debug, Serialize)]
pub struct StreamResponse {
    pub stream_url: String,
    pub content_type: String,
    pub content_length: Option<u64>,
}

/// OAuth refresh response
#[derive(Debug, Serialize)]
pub struct OAuthRefreshResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
}

/// REST API errors
#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl RestError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            RestError::Configuration(_) => 400,
            RestError::Authentication(_) => 401,
            RestError::NotFound(_) => 404,
            RestError::NotImplemented(_) => 501,
            RestError::Internal(_) => 500,
        }
    }
}

/// Minimal configuration request (from Java REST endpoints)
#[derive(Debug, Deserialize)]
pub struct MinimalConfigRequest {
    pub enabled: Option<bool>,
    pub clients: Option<Vec<String>>,
    pub oauth: Option<MinimalOAuthConfig>,
    pub pot: Option<MinimalPotConfig>,
}

/// Minimal configuration response
#[derive(Debug, Serialize)]
pub struct MinimalConfigResponse {
    pub enabled: bool,
    pub clients: Vec<String>,
    pub oauth_enabled: bool,
    pub pot_configured: bool,
}

/// Minimal OAuth configuration
#[derive(Debug, Deserialize)]
pub struct MinimalOAuthConfig {
    pub enabled: bool,
    pub refresh_token: Option<String>,
}

/// Minimal PoToken configuration
#[derive(Debug, Deserialize)]
pub struct MinimalPotConfig {
    pub token: Option<String>,
    pub visitor_data: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_update_request_deserialization() {
        let json = r#"{
            "refreshToken": "test_token",
            "skipInitialization": true,
            "poToken": "test_po_token",
            "visitorData": "test_visitor_data"
        }"#;
        
        let request: ConfigUpdateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.refresh_token, Some("test_token".to_string()));
        assert_eq!(request.skip_initialization, Some(true));
        assert_eq!(request.po_token, Some("test_po_token".to_string()));
        assert_eq!(request.visitor_data, Some("test_visitor_data".to_string()));
    }
    
    #[test]
    fn test_config_response_serialization() {
        let response = ConfigResponse {
            refresh_token: None,
            has_refresh_token: true,
            po_token_configured: false,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("hasRefreshToken"));
        assert!(json.contains("poTokenConfigured"));
    }
    
    #[test]
    fn test_rest_error_status_codes() {
        assert_eq!(RestError::Configuration("test".to_string()).status_code(), 400);
        assert_eq!(RestError::Authentication("test".to_string()).status_code(), 401);
        assert_eq!(RestError::NotFound("test".to_string()).status_code(), 404);
        assert_eq!(RestError::NotImplemented("test".to_string()).status_code(), 501);
        assert_eq!(RestError::Internal("test".to_string()).status_code(), 500);
    }
}
