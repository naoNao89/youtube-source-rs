use crate::error::{Result, YoutubeError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};
use tokio::time::interval;
use uuid::Uuid;

/// OAuth2 constants from Java implementation
const CLIENT_ID: &str = "861556708454-d6dlm3lh05idd8npek18k6be8ba3oc68.apps.googleusercontent.com";
const CLIENT_SECRET: &str = "SboVhoG9s0rNafixCSGGKXAT";
const SCOPES: &str = "http://gdata.youtube.com https://www.googleapis.com/auth/youtube";
const OAUTH_FETCH_CONTEXT_ATTRIBUTE: &str = "yt-oauth";
// Removed unused constant OAUTH_INJECT_CONTEXT_ATTRIBUTE

/// Access token structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub token_type: String,
    pub expires_at: SystemTime,
    pub refresh_token: Option<String>,
}

impl AccessToken {
    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }

    pub fn expires_in_seconds(&self) -> u64 {
        self.expires_at
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::ZERO)
            .as_secs()
    }
}

/// Device code response from OAuth2 device flow
#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    verification_url: String,
    user_code: String,
    device_code: String,
    interval: Option<u64>,
    _expires_in: Option<u64>,
}

/// Token response from OAuth2 token endpoint
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
    error: Option<String>,
}

/// YouTube OAuth2 Handler - migrated from YoutubeOauth2Handler.java
///
/// Implements OAuth2 device flow for YouTube API authentication
/// Features:
/// - Device code flow for user authorization
/// - Automatic token refresh
/// - Token application to HTTP requests
/// - Error handling and retry logic
#[derive(Debug)]
pub struct YoutubeOauth2Handler {
    http_client: reqwest::Client,
    enabled: Arc<RwLock<bool>>,
    refresh_token: Arc<RwLock<Option<String>>>,
    access_token: Arc<RwLock<Option<AccessToken>>>,
    fetch_error_count: Arc<Mutex<u32>>,
}

impl Default for YoutubeOauth2Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubeOauth2Handler {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            enabled: Arc::new(RwLock::new(false)),
            refresh_token: Arc::new(RwLock::new(None)),
            access_token: Arc::new(RwLock::new(None)),
            fetch_error_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Set refresh token and initialize OAuth2 flow
    /// Migrated from setRefreshToken() in Java
    pub async fn set_refresh_token(
        &self,
        refresh_token: Option<String>,
        skip_initialization: bool,
    ) -> Result<()> {
        {
            let mut token = self.refresh_token.write().await;
            *token = refresh_token.clone();
        }

        // Clear existing access token
        {
            let mut access = self.access_token.write().await;
            *access = None;
        }

        if let Some(ref token) = refresh_token {
            if !token.trim().is_empty() {
                self.refresh_access_token(true).await?;

                let mut enabled = self.enabled.write().await;
                *enabled = true;
                return Ok(());
            }
        }

        if !skip_initialization {
            self.initialize_access_token().await?;
        }

        Ok(())
    }

    /// Check if access token should be refreshed
    /// Migrated from shouldRefreshAccessToken() in Java
    pub async fn should_refresh_access_token(&self) -> bool {
        let enabled = *self.enabled.read().await;
        let refresh_token = self.refresh_token.read().await;
        let access_token = self.access_token.read().await;

        enabled
            && refresh_token.as_ref().is_some_and(|t| !t.trim().is_empty())
            && access_token.as_ref().map_or(true, |t| t.is_expired())
    }

    /// Get current refresh token
    pub async fn get_refresh_token(&self) -> Option<String> {
        self.refresh_token.read().await.clone()
    }

    /// Check if this is an OAuth fetch context
    pub fn is_oauth_fetch_context(&self, context: &HashMap<String, String>) -> bool {
        context.get(OAUTH_FETCH_CONTEXT_ATTRIBUTE) == Some(&"true".to_string())
    }

    /// Initialize access token using device flow
    /// Migrated from initializeAccessToken() in Java
    async fn initialize_access_token(&self) -> Result<()> {
        let device_response = self.fetch_device_code().await?;

        log::debug!("Device code response: {device_response:?}");

        let verification_url = device_response.verification_url;
        let user_code = device_response.user_code;
        let device_code = device_response.device_code;
        let interval = device_response.interval.unwrap_or(5) * 1000; // Convert to milliseconds

        log::info!("==================================================");
        log::info!("!!! DO NOT AUTHORISE WITH YOUR MAIN ACCOUNT, USE A BURNER !!!");
        log::info!(
            "OAUTH INTEGRATION: To give youtube-source access to your account, go to {verification_url} and enter code {user_code}"
        );
        log::info!("!!! DO NOT AUTHORISE WITH YOUR MAIN ACCOUNT, USE A BURNER !!!");
        log::info!("==================================================");

        // Start polling for token in background
        let handler = self.clone();
        tokio::spawn(async move {
            if let Err(e) = handler.poll_for_token(device_code, interval).await {
                log::error!("Failed to poll for OAuth2 token: {e}");
            }
        });

        Ok(())
    }

    /// Fetch device code from YouTube OAuth2 endpoint
    /// Migrated from fetchDeviceCode() in Java
    async fn fetch_device_code(&self) -> Result<DeviceCodeResponse> {
        let request_body = json!({
            "client_id": CLIENT_ID,
            "scope": SCOPES,
            "device_id": Uuid::new_v4().to_string().replace("-", ""),
            "device_model": "ytlr::"
        });

        let response = self
            .http_client
            .post("https://www.youtube.com/o/oauth2/device/code")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to fetch device code: {e}")))?;

        if !response.status().is_success() {
            return Err(YoutubeError::HttpError(format!(
                "Device code fetch failed with status: {}",
                response.status()
            )));
        }

        let device_response: DeviceCodeResponse = response.json().await.map_err(|e| {
            YoutubeError::ParseError(format!("Failed to parse device code response: {e}"))
        })?;

        Ok(device_response)
    }

    /// Poll for OAuth2 token
    /// Migrated from pollForToken() in Java
    async fn poll_for_token(&self, device_code: String, interval_ms: u64) -> Result<()> {
        let request_body = json!({
            "client_id": CLIENT_ID,
            "client_secret": CLIENT_SECRET,
            "code": device_code,
            "grant_type": "http://oauth.net/grant_type/device/1.0"
        });

        let mut interval_timer = interval(Duration::from_millis(interval_ms));

        loop {
            interval_timer.tick().await;

            let response = self
                .http_client
                .post("https://www.youtube.com/o/oauth2/token")
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await
                .map_err(|e| YoutubeError::HttpError(format!("Failed to poll for token: {e}")))?;

            if !response.status().is_success() {
                log::warn!("Token polling failed with status: {}", response.status());
                continue;
            }

            let token_response: TokenResponse = response.json().await.map_err(|e| {
                YoutubeError::ParseError(format!("Failed to parse token response: {e}"))
            })?;

            log::debug!("OAuth2 token response: {token_response:?}");

            if let Some(error) = token_response.error {
                match error.as_str() {
                    "authorization_pending" | "slow_down" => {
                        continue;
                    }
                    "expired_token" => {
                        log::error!(
                            "OAUTH INTEGRATION: The device token has expired. OAuth integration has been canceled."
                        );
                        return Err(YoutubeError::AuthError("Device token expired".to_string()));
                    }
                    "access_denied" => {
                        log::error!(
                            "OAUTH INTEGRATION: Account linking was denied. OAuth integration has been canceled."
                        );
                        return Err(YoutubeError::AuthError("Access denied".to_string()));
                    }
                    _ => {
                        log::error!("Unhandled OAuth2 error: {error}");
                        return Err(YoutubeError::AuthError(format!("OAuth2 error: {error}")));
                    }
                }
            }

            // Success - update tokens
            self.update_tokens(token_response).await?;

            let refresh_token = self.refresh_token.read().await;
            log::info!(
                "OAUTH INTEGRATION: Token retrieved successfully. Store your refresh token as this can be reused. ({})",
                refresh_token.as_deref().unwrap_or("None")
            );

            let mut enabled = self.enabled.write().await;
            *enabled = true;

            return Ok(());
        }
    }

    /// Refresh access token using refresh token
    /// Migrated from refreshAccessToken() in Java
    pub async fn refresh_access_token(&self, force: bool) -> Result<()> {
        log::debug!("Refreshing access token (force: {force})");

        let refresh_token = {
            let token = self.refresh_token.read().await;
            token.clone()
        };

        let refresh_token = match refresh_token {
            Some(token) if !token.trim().is_empty() => token,
            _ => {
                return Err(YoutubeError::AuthError(
                    "Cannot fetch access token without a refresh token".to_string(),
                ));
            }
        };

        if !self.should_refresh_access_token().await && !force {
            log::debug!("Access token does not need to be refreshed yet.");
            return Ok(());
        }

        // Double-checked locking pattern
        let should_refresh = {
            let token = self.refresh_token.read().await;
            token.as_ref().is_some_and(|t| !t.trim().is_empty())
                && (self.should_refresh_access_token().await || force)
        };

        if !should_refresh {
            log::debug!("Access token does not need to be refreshed yet (double-check).");
            return Ok(());
        }

        match self.create_new_access_token(&refresh_token).await {
            Ok(token_response) => {
                self.update_tokens(token_response).await?;
                log::info!("YouTube access token refreshed successfully");

                // Reset error count on success
                let mut error_count = self.fetch_error_count.lock().await;
                *error_count = 0;

                Ok(())
            }
            Err(e) => {
                log::error!("Failed to refresh access token: {e}");
                Err(e)
            }
        }
    }

    /// Create new access token using refresh token
    /// Migrated from createNewAccessToken() in Java
    async fn create_new_access_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        let request_body = json!({
            "client_id": CLIENT_ID,
            "client_secret": CLIENT_SECRET,
            "refresh_token": refresh_token,
            "grant_type": "refresh_token"
        });

        let response = self
            .http_client
            .post("https://www.youtube.com/o/oauth2/token")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to refresh token: {e}")))?;

        if !response.status().is_success() {
            return Err(YoutubeError::HttpError(format!(
                "Token refresh failed with status: {}",
                response.status()
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            YoutubeError::ParseError(format!("Failed to parse token refresh response: {e}"))
        })?;

        if let Some(error) = &token_response.error {
            return Err(YoutubeError::AuthError(format!(
                "Refreshing access token returned error: {error}"
            )));
        }

        Ok(token_response)
    }

    /// Update internal tokens from response
    /// Migrated from updateTokens() in Java
    async fn update_tokens(&self, token_response: TokenResponse) -> Result<()> {
        let expires_at = SystemTime::now() + Duration::from_secs(token_response.expires_in)
            - Duration::from_secs(60); // 1 minute buffer

        let access_token = AccessToken {
            token: token_response.access_token.clone(),
            token_type: token_response.token_type.clone(),
            expires_at,
            refresh_token: token_response.refresh_token.clone(),
        };

        // Update access token
        {
            let mut token = self.access_token.write().await;
            *token = Some(access_token);
        }

        // Update refresh token if provided
        if let Some(new_refresh_token) = token_response.refresh_token {
            let mut refresh_token = self.refresh_token.write().await;
            *refresh_token = Some(new_refresh_token);
        }

        log::debug!(
            "OAuth access token updated. Expires in {} seconds.",
            token_response.expires_in
        );

        Ok(())
    }

    /// Apply OAuth token to HTTP request
    /// Migrated from applyToken() in Java
    pub async fn apply_token(&self, request: &mut reqwest::Request) -> Result<()> {
        let enabled = *self.enabled.read().await;
        let has_refresh_token = {
            let token = self.refresh_token.read().await;
            token.as_ref().is_some_and(|t| !t.trim().is_empty())
        };

        if !enabled || !has_refresh_token {
            return Ok(());
        }

        if self.should_refresh_access_token().await {
            log::debug!("Access token has expired, refreshing...");

            if let Err(e) = self.refresh_access_token(false).await {
                let mut error_count = self.fetch_error_count.lock().await;
                *error_count += 1;

                if *error_count <= 3 {
                    log::error!("Refreshing YouTube access token failed: {e}");
                } else {
                    log::debug!("Refreshing YouTube access token failed: {e}");
                }

                // Set token expiry to retry in 15 seconds
                let retry_time = SystemTime::now() + Duration::from_secs(15);
                if let Some(ref mut token) = *self.access_token.write().await {
                    token.expires_at = retry_time;
                }

                return Ok(());
            }
        }

        // Apply token if available and valid
        let access_token = self.access_token.read().await;
        if let Some(ref token) = *access_token {
            if !token.is_expired() {
                let auth_header = format!("{} {}", token.token_type, token.token);
                log::debug!("Using oauth authorization header: {auth_header}");

                request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    auth_header.parse().map_err(|e| {
                        YoutubeError::HttpError(format!("Invalid auth header: {e}"))
                    })?,
                );
            }
        }

        Ok(())
    }

    /// Apply specific token to request (for manual token injection)
    pub fn apply_token_direct(request: &mut reqwest::Request, token: &str) -> Result<()> {
        let auth_header = format!("Bearer {token}");
        request.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            auth_header
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid auth header: {e}")))?,
        );
        Ok(())
    }
}

// Clone implementation for spawning async tasks
impl Clone for YoutubeOauth2Handler {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            enabled: Arc::clone(&self.enabled),
            refresh_token: Arc::clone(&self.refresh_token),
            access_token: Arc::clone(&self.access_token),
            fetch_error_count: Arc::clone(&self.fetch_error_count),
        }
    }
}

/// YouTube Access Token Tracker - migrated from YoutubeAccessTokenTracker.java
///
/// Manages visitor IDs and token fetch contexts for YouTube API requests
/// Features:
/// - Periodic visitor ID refresh
/// - Token fetch context management
/// - Android client integration for visitor ID fetching
const TOKEN_FETCH_CONTEXT_ATTRIBUTE: &str = "yt-raw";
const VISITOR_ID_REFRESH_INTERVAL: Duration = Duration::from_secs(10 * 60); // 10 minutes

#[derive(Debug)]
pub struct YoutubeAccessTokenTracker {
    http_client: reqwest::Client,
    visitor_id: Arc<RwLock<Option<String>>>,
    last_visitor_id_update: Arc<RwLock<SystemTime>>,
    token_lock: Arc<Mutex<()>>,
}

impl Default for YoutubeAccessTokenTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubeAccessTokenTracker {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            visitor_id: Arc::new(RwLock::new(None)),
            last_visitor_id_update: Arc::new(RwLock::new(UNIX_EPOCH)),
            token_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Get visitor ID, refreshing if necessary
    /// Migrated from getVisitorId() in Java
    pub async fn get_visitor_id(&self) -> Option<String> {
        let now = SystemTime::now();

        // Check if we need to update
        let should_update = {
            let last_update = *self.last_visitor_id_update.read().await;
            let visitor_id = self.visitor_id.read().await;

            visitor_id.is_none()
                || now.duration_since(last_update).unwrap_or(Duration::MAX)
                    >= VISITOR_ID_REFRESH_INTERVAL
        };

        if should_update {
            let _lock = self.token_lock.lock().await;

            // Double-check after acquiring lock
            let last_update = *self.last_visitor_id_update.read().await;
            if now.duration_since(last_update).unwrap_or(Duration::MAX)
                < VISITOR_ID_REFRESH_INTERVAL
            {
                log::debug!(
                    "YouTube visitor id was recently updated, not updating again right away."
                );
                return self.visitor_id.read().await.clone();
            }

            // Update timestamp first
            {
                let mut last_update = self.last_visitor_id_update.write().await;
                *last_update = now;
            }

            // Fetch new visitor ID
            match self.fetch_visitor_id().await {
                Ok(new_visitor_id) => {
                    {
                        let mut visitor_id = self.visitor_id.write().await;
                        *visitor_id = Some(new_visitor_id.clone());
                    }

                    log::info!(
                        "Updating YouTube visitor id succeeded, new one is {}, next update will be after {} seconds.",
                        new_visitor_id,
                        VISITOR_ID_REFRESH_INTERVAL.as_secs()
                    );
                }
                Err(e) => {
                    log::error!("YouTube visitor id update failed: {e}");
                }
            }
        }

        self.visitor_id.read().await.clone()
    }

    /// Check if this is a token fetch context
    pub fn is_token_fetch_context(&self, context: &HashMap<String, String>) -> bool {
        context.get(TOKEN_FETCH_CONTEXT_ATTRIBUTE) == Some(&"true".to_string())
    }

    /// Fetch visitor ID from YouTube API
    /// Migrated from fetchVisitorId() in Java
    async fn fetch_visitor_id(&self) -> Result<String> {
        // Create Android client configuration for visitor ID request
        let client_config = json!({
            "context": {
                "client": {
                    "clientName": "ANDROID",
                    "clientVersion": "19.09.37",
                    "androidSdkVersion": 30,
                    "userAgent": "com.google.android.youtube/19.09.37 (Linux; U; Android 11) gzip",
                    "osName": "Android",
                    "osVersion": "11"
                }
            }
        });

        let response = self
            .http_client
            .post("https://youtubei.googleapis.com/youtubei/v1/visitor_id")
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                "com.google.android.youtube/19.09.37 (Linux; U; Android 11) gzip",
            )
            .json(&client_config)
            .send()
            .await
            .map_err(|e| YoutubeError::HttpError(format!("Failed to fetch visitor ID: {e}")))?;

        if !response.status().is_success() {
            return Err(YoutubeError::HttpError(format!(
                "Visitor ID fetch failed with status: {}",
                response.status()
            )));
        }

        let json: Value = response.json().await.map_err(|e| {
            YoutubeError::ParseError(format!("Failed to parse visitor ID response: {e}"))
        })?;

        let visitor_id = json
            .get("responseContext")
            .and_then(|ctx| ctx.get("visitorData"))
            .and_then(|data| data.as_str())
            .ok_or_else(|| {
                YoutubeError::ParseError("Visitor ID not found in response".to_string())
            })?;

        Ok(visitor_id.to_string())
    }
}

impl Clone for YoutubeAccessTokenTracker {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            visitor_id: Arc::clone(&self.visitor_id),
            last_visitor_id_update: Arc::clone(&self.last_visitor_id_update),
            token_lock: Arc::clone(&self.token_lock),
        }
    }
}
