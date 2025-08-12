use super::{Pot, YoutubeOauthConfig};
use crate::ClientOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// YouTube plugin configuration
///
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/YoutubeConfig.java`
///
/// This configuration is used by the Lavalink plugin to configure YouTube source behavior.
/// It supports all the same options as the Java implementation with additional Rust-specific optimizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeConfig {
    /// Whether this source can be used
    pub enabled: bool,

    /// Whether "ytsearch:" and "ytmsearch:" can be used
    pub allow_search: bool,

    /// Whether just video IDs can match. If false, only complete URLs will be loaded
    pub allow_direct_video_ids: bool,

    /// Whether just playlist IDs can match. If false, only complete URLs will be loaded
    pub allow_direct_playlist_ids: bool,

    /// PoToken configuration for bypassing bot detection
    pub pot: Option<Pot>,

    /// The clients to use for track loading
    /// Clients are queried in the order they are given
    pub clients: Option<Vec<String>>,

    /// Client-specific options for fine-tuning behavior
    pub client_options: HashMap<String, ClientOptions>,

    /// OAuth configuration for authenticated access
    pub oauth: Option<YoutubeOauthConfig>,
}

impl Default for YoutubeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_search: true,
            allow_direct_video_ids: true,
            allow_direct_playlist_ids: true,
            pot: None,
            clients: None,
            client_options: HashMap::new(),
            oauth: None,
        }
    }
}

impl YoutubeConfig {
    /// Create a new YouTube configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether the source is enabled
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether search is allowed
    pub fn set_allow_search(mut self, allow_search: bool) -> Self {
        self.allow_search = allow_search;
        self
    }

    /// Set whether direct video IDs are allowed
    pub fn set_allow_direct_video_ids(mut self, allow_direct_video_ids: bool) -> Self {
        self.allow_direct_video_ids = allow_direct_video_ids;
        self
    }

    /// Set whether direct playlist IDs are allowed
    pub fn set_allow_direct_playlist_ids(mut self, allow_direct_playlist_ids: bool) -> Self {
        self.allow_direct_playlist_ids = allow_direct_playlist_ids;
        self
    }

    /// Set the PoToken configuration
    pub fn set_pot(mut self, pot: Option<Pot>) -> Self {
        self.pot = pot;
        self
    }

    /// Set the clients to use
    pub fn set_clients(mut self, clients: Vec<String>) -> Self {
        self.clients = Some(clients);
        self
    }

    /// Add client-specific options
    pub fn add_client_options(mut self, client_name: String, options: ClientOptions) -> Self {
        self.client_options.insert(client_name, options);
        self
    }

    /// Set OAuth configuration
    pub fn set_oauth(mut self, oauth: Option<YoutubeOauthConfig>) -> Self {
        self.oauth = oauth;
        self
    }

    /// Get options for a specific client, returning default if not configured
    pub fn get_options_for_client(&self, client_name: &str) -> ClientOptions {
        self.client_options
            .get(client_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if the plugin is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if search is allowed
    pub fn is_search_allowed(&self) -> bool {
        self.allow_search
    }

    /// Check if direct video IDs are allowed
    pub fn are_direct_video_ids_allowed(&self) -> bool {
        self.allow_direct_video_ids
    }

    /// Check if direct playlist IDs are allowed
    pub fn are_direct_playlist_ids_allowed(&self) -> bool {
        self.allow_direct_playlist_ids
    }

    /// Get the configured clients, or default clients if none specified
    pub fn get_clients(&self) -> Vec<String> {
        self.clients.clone().unwrap_or_else(|| {
            vec![
                "MUSIC".to_string(),
                "ANDROID_VR".to_string(),
                "WEB".to_string(),
                "WEBEMBEDDED".to_string(),
            ]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = YoutubeConfig::default();
        assert!(config.enabled);
        assert!(config.allow_search);
        assert!(config.allow_direct_video_ids);
        assert!(config.allow_direct_playlist_ids);
        assert!(config.pot.is_none());
        assert!(config.oauth.is_none());
    }

    #[test]
    fn test_builder_pattern() {
        let config = YoutubeConfig::new()
            .set_enabled(false)
            .set_allow_search(false)
            .set_clients(vec!["WEB".to_string(), "MUSIC".to_string()]);

        assert!(!config.enabled);
        assert!(!config.allow_search);
        assert_eq!(config.get_clients(), vec!["WEB", "MUSIC"]);
    }

    #[test]
    fn test_get_options_for_client() {
        let mut config = YoutubeConfig::new();
        let custom_options = ClientOptions::default();
        config
            .client_options
            .insert("WEB".to_string(), custom_options.clone());

        assert_eq!(config.get_options_for_client("WEB"), custom_options);
        assert_eq!(
            config.get_options_for_client("NONEXISTENT"),
            ClientOptions::default()
        );
    }
}
