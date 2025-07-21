use serde::{Deserialize, Serialize};

/// OAuth configuration for YouTube plugin
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/YoutubeOauthConfig.java`
/// 
/// This configuration enables OAuth2 authentication with YouTube to bypass bot detection
/// and access age-restricted or private content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct YoutubeOauthConfig {
    /// Whether OAuth is enabled
    pub enabled: bool,
    
    /// The refresh token for OAuth authentication
    /// If provided, skips the OAuth flow entirely
    pub refresh_token: Option<String>,
    
    /// Whether to skip OAuth initialization
    /// Set this if you don't want the OAuth flow to be triggered immediately
    pub skip_initialization: bool,
}


impl YoutubeOauthConfig {
    /// Create a new OAuth configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create an enabled OAuth configuration
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            refresh_token: None,
            skip_initialization: false,
        }
    }
    
    /// Create an OAuth configuration with a refresh token
    pub fn with_refresh_token(refresh_token: String) -> Self {
        Self {
            enabled: true,
            refresh_token: Some(refresh_token),
            skip_initialization: false,
        }
    }
    
    /// Set whether OAuth is enabled
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Set the refresh token
    pub fn set_refresh_token(mut self, refresh_token: Option<String>) -> Self {
        self.refresh_token = refresh_token;
        self
    }
    
    /// Set whether to skip initialization
    pub fn set_skip_initialization(mut self, skip_initialization: bool) -> Self {
        self.skip_initialization = skip_initialization;
        self
    }
    
    /// Check if OAuth is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get the refresh token
    pub fn get_refresh_token(&self) -> Option<&String> {
        self.refresh_token.as_ref()
    }
    
    /// Check if initialization should be skipped
    pub fn should_skip_initialization(&self) -> bool {
        self.skip_initialization
    }
    
    /// Check if a refresh token is available
    pub fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }
    
    /// Check if OAuth should be initialized immediately
    /// Returns true if OAuth is enabled and initialization is not skipped
    pub fn should_initialize(&self) -> bool {
        self.enabled && !self.skip_initialization
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_oauth_config() {
        let config = YoutubeOauthConfig::default();
        assert!(!config.is_enabled());
        assert!(!config.has_refresh_token());
        assert!(!config.should_skip_initialization());
        assert!(!config.should_initialize());
    }
    
    #[test]
    fn test_enabled_oauth_config() {
        let config = YoutubeOauthConfig::enabled();
        assert!(config.is_enabled());
        assert!(!config.has_refresh_token());
        assert!(!config.should_skip_initialization());
        assert!(config.should_initialize());
    }
    
    #[test]
    fn test_oauth_with_refresh_token() {
        let config = YoutubeOauthConfig::with_refresh_token("test_token".to_string());
        assert!(config.is_enabled());
        assert!(config.has_refresh_token());
        assert_eq!(config.get_refresh_token(), Some(&"test_token".to_string()));
        assert!(config.should_initialize());
    }
    
    #[test]
    fn test_skip_initialization() {
        let config = YoutubeOauthConfig::enabled()
            .set_skip_initialization(true);
            
        assert!(config.is_enabled());
        assert!(config.should_skip_initialization());
        assert!(!config.should_initialize());
    }
    
    #[test]
    fn test_builder_pattern() {
        let config = YoutubeOauthConfig::new()
            .set_enabled(true)
            .set_refresh_token(Some("token".to_string()))
            .set_skip_initialization(true);
            
        assert!(config.is_enabled());
        assert!(config.has_refresh_token());
        assert!(config.should_skip_initialization());
    }
}
