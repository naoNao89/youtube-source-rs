// use std::collections::HashMap; // Currently unused
use crate::error::YoutubeError;
use cookie_store::CookieStore;
use reqwest::{Client, Request, Response};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// HTTP context attributes for request metadata
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    pub visitor_id: Option<String>,
    pub oauth_token: Option<String>,
    pub client_name: Option<String>,
    pub referer: Option<String>,
    pub is_music_request: bool,
    pub is_player_request: bool,
    pub is_search_request: bool,
    pub is_browse_request: bool,
    pub is_next_request: bool,
}

/// Rate limiting state tracking
#[derive(Debug)]
struct RateLimitState {
    last_429_time: Option<Instant>,
    backoff_duration: Duration,
    consecutive_429s: u32,
}

impl Default for RateLimitState {
    fn default() -> Self {
        Self {
            last_429_time: None,
            backoff_duration: Duration::from_secs(1),
            consecutive_429s: 0,
        }
    }
}

/// YouTube HTTP Context Filter - Middleware for YouTube API requests
///
/// Based on Java YoutubeHttpContextFilter.java, this provides:
/// - Cookie management with automatic clearing per request sequence
/// - User-Agent and Visitor-ID header injection
/// - OAuth2 token application for authenticated requests
/// - Rate limiting detection (429 status code handling)
/// - Connection reset retry logic
/// - Request context attribute management
#[derive(Debug)]
pub struct YoutubeHttpContextFilter {
    cookie_store: Arc<RwLock<CookieStore>>,
    rate_limit_state: Arc<RwLock<RateLimitState>>,
    visitor_id_tracker: Arc<RwLock<Option<String>>>,
}

impl YoutubeHttpContextFilter {
    pub fn new() -> Self {
        Self {
            cookie_store: Arc::new(RwLock::new(CookieStore::default())),
            rate_limit_state: Arc::new(RwLock::new(RateLimitState::default())),
            visitor_id_tracker: Arc::new(RwLock::new(None)),
        }
    }

    /// Apply request filtering - inject headers and manage context
    pub async fn apply_request_filter(
        &self,
        mut request: Request,
        context: &RequestContext,
    ) -> Result<Request, YoutubeError> {
        // Apply User-Agent based on client type
        if let Some(client_name) = &context.client_name {
            let user_agent = self.get_user_agent_for_client(client_name);
            request.headers_mut().insert(
                reqwest::header::USER_AGENT,
                user_agent
                    .parse()
                    .map_err(|e| YoutubeError::HttpError(format!("Invalid user agent: {e}")))?,
            );
        }

        // Apply Visitor-ID header if available
        if let Some(visitor_id) = &context.visitor_id {
            request.headers_mut().insert(
                "X-Goog-Visitor-Id",
                visitor_id
                    .parse()
                    .map_err(|e| YoutubeError::HttpError(format!("Invalid visitor ID: {e}")))?,
            );
        }

        // Apply OAuth token for player requests
        if context.is_player_request {
            if let Some(oauth_token) = &context.oauth_token {
                request.headers_mut().insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {oauth_token}").parse().map_err(|e| {
                        YoutubeError::HttpError(format!("Invalid OAuth token: {e}"))
                    })?,
                );
            }
        }

        // Apply Referer header for music requests
        if context.is_music_request {
            let referer = context
                .referer
                .as_deref()
                .unwrap_or("https://music.youtube.com/");
            request.headers_mut().insert(
                reqwest::header::REFERER,
                referer
                    .parse()
                    .map_err(|e| YoutubeError::HttpError(format!("Invalid referer: {e}")))?,
            );
        }

        // Apply standard YouTube API headers
        self.apply_standard_headers(&mut request)?;

        Ok(request)
    }

    /// Apply response filtering - handle rate limiting and errors
    pub async fn apply_response_filter(
        &self,
        response: Response,
        context: &RequestContext,
    ) -> Result<Response, YoutubeError> {
        let status = response.status();

        // Handle rate limiting (429 status)
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            self.handle_rate_limit().await?;
            return Err(YoutubeError::RateLimited(
                "YouTube API rate limit exceeded".to_string(),
            ));
        }

        // Reset rate limit state on successful request
        if status.is_success() {
            let mut rate_limit = self.rate_limit_state.write().await;
            rate_limit.consecutive_429s = 0;
            rate_limit.backoff_duration = Duration::from_secs(1);
        }

        // Clear cookies after request sequence (mimics Java behavior)
        if context.is_player_request {
            self.clear_cookies().await;
        }

        Ok(response)
    }

    /// Handle rate limiting with exponential backoff
    async fn handle_rate_limit(&self) -> Result<(), YoutubeError> {
        let mut rate_limit = self.rate_limit_state.write().await;

        rate_limit.last_429_time = Some(Instant::now());
        rate_limit.consecutive_429s += 1;

        // Exponential backoff: 1s, 2s, 4s, 8s, max 60s
        let backoff_secs = std::cmp::min(1u64 << (rate_limit.consecutive_429s - 1), 60);
        rate_limit.backoff_duration = Duration::from_secs(backoff_secs);

        log::warn!(
            "Rate limited by YouTube API. Backing off for {} seconds (attempt {})",
            backoff_secs,
            rate_limit.consecutive_429s
        );

        Ok(())
    }

    /// Clear cookies (mimics Java BasicCookieStore clearing)
    async fn clear_cookies(&self) {
        let mut store = self.cookie_store.write().await;
        *store = CookieStore::default();
        log::debug!("Cleared cookie store after request sequence");
    }

    /// Apply standard YouTube API headers
    fn apply_standard_headers(&self, request: &mut Request) -> Result<(), YoutubeError> {
        let headers = request.headers_mut();

        // Standard headers for YouTube API
        headers.insert(
            reqwest::header::ACCEPT,
            "*/*"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid accept header: {e}")))?,
        );

        headers.insert(
            reqwest::header::ACCEPT_LANGUAGE,
            "en-US,en;q=0.9"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid accept-language: {e}")))?,
        );

        headers.insert(
            "Accept-Encoding",
            "gzip, deflate, br"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid accept-encoding: {e}")))?,
        );

        headers.insert(
            "Cache-Control",
            "no-cache"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid cache-control: {e}")))?,
        );

        headers.insert(
            "Pragma",
            "no-cache"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid pragma: {e}")))?,
        );

        headers.insert(
            "Sec-Fetch-Dest",
            "empty"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid sec-fetch-dest: {e}")))?,
        );

        headers.insert(
            "Sec-Fetch-Mode",
            "cors"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid sec-fetch-mode: {e}")))?,
        );

        headers.insert(
            "Sec-Fetch-Site",
            "same-origin"
                .parse()
                .map_err(|e| YoutubeError::HttpError(format!("Invalid sec-fetch-site: {e}")))?,
        );

        Ok(())
    }

    /// Get appropriate User-Agent for client type
    fn get_user_agent_for_client(&self, client_name: &str) -> String {
        match client_name {
            "WEB" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "WEB_REMIX" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "ANDROID" => "com.google.android.youtube/19.44.38 (Linux; U; Android 11) gzip".to_string(),
            "ANDROID_MUSIC" => "com.google.android.apps.youtube.music/6.42.52 (Linux; U; Android 11) gzip".to_string(),
            "IOS" => "com.google.ios.youtube/19.44.7 (iPhone16,2; U; CPU iOS 17_7_2 like Mac OS X)".to_string(),
            "TV" => "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string(),
            _ => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0 Safari/537.36".to_string(),
        }
    }

    /// Check if we should wait due to rate limiting
    pub async fn should_wait_for_rate_limit(&self) -> Option<Duration> {
        let rate_limit = self.rate_limit_state.read().await;

        if let Some(last_429) = rate_limit.last_429_time {
            let elapsed = last_429.elapsed();
            if elapsed < rate_limit.backoff_duration {
                return Some(rate_limit.backoff_duration - elapsed);
            }
        }

        None
    }

    /// Set visitor ID for future requests
    pub async fn set_visitor_id(&self, visitor_id: String) {
        let mut tracker = self.visitor_id_tracker.write().await;
        *tracker = Some(visitor_id);
        log::debug!("Updated visitor ID for future requests");
    }

    /// Get current visitor ID
    pub async fn get_visitor_id(&self) -> Option<String> {
        let tracker = self.visitor_id_tracker.read().await;
        tracker.clone()
    }
}

impl Default for YoutubeHttpContextFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP Client wrapper with YouTube-specific filtering
#[derive(Debug, Clone)]
pub struct YoutubeHttpClient {
    client: Client,
    filter: Arc<YoutubeHttpContextFilter>,
}

impl YoutubeHttpClient {
    pub fn new() -> Result<Self, YoutubeError> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| YoutubeError::HttpError(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            filter: Arc::new(YoutubeHttpContextFilter::new()),
        })
    }

    /// Execute request with YouTube-specific filtering
    pub async fn execute_with_context(
        &self,
        request: Request,
        context: RequestContext,
    ) -> Result<Response, YoutubeError> {
        // Check rate limiting
        if let Some(wait_duration) = self.filter.should_wait_for_rate_limit().await {
            log::info!(
                "Waiting {} seconds due to rate limiting",
                wait_duration.as_secs()
            );
            tokio::time::sleep(wait_duration).await;
        }

        // Apply request filtering
        let filtered_request = self.filter.apply_request_filter(request, &context).await?;

        // Execute request with retry logic
        let response = self.execute_with_retry(filtered_request).await?;

        // Apply response filtering
        self.filter.apply_response_filter(response, &context).await
    }

    /// Execute request with connection reset retry logic
    async fn execute_with_retry(&self, request: Request) -> Result<Response, YoutubeError> {
        const MAX_RETRIES: u32 = 3;
        let mut last_error = None;
        let mut current_request = Some(request);

        for attempt in 0..MAX_RETRIES {
            // Get the request for this attempt
            let req = if attempt == 0 {
                // First attempt: use the original request
                current_request.take().unwrap()
            } else {
                // Subsequent attempts: try to clone the original request
                match current_request.as_ref().and_then(|r| r.try_clone()) {
                    Some(cloned) => cloned,
                    None => {
                        // If we can't clone, we can't retry
                        log::warn!("Cannot clone request for retry, stopping retries");
                        break;
                    }
                }
            };

            match self.client.execute(req).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);

                    // Only retry on connection errors and if we have more attempts
                    if attempt < MAX_RETRIES - 1 {
                        let wait_time = Duration::from_millis(100 * (1 << attempt)); // 100ms, 200ms, 400ms
                        log::warn!(
                            "Request failed (attempt {}), retrying in {:?}: {}",
                            attempt + 1,
                            wait_time,
                            last_error.as_ref().unwrap()
                        );
                        tokio::time::sleep(wait_time).await;
                    }
                }
            }
        }

        Err(YoutubeError::HttpError(format!(
            "Request failed after {} attempts: {}",
            MAX_RETRIES,
            last_error.unwrap()
        )))
    }

    /// Get the underlying HTTP client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the HTTP filter
    pub fn filter(&self) -> &Arc<YoutubeHttpContextFilter> {
        &self.filter
    }
}

impl Default for YoutubeHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default YouTube HTTP client")
    }
}
