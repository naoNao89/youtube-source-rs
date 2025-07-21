use log::{debug, info, warn};
use crate::{YoutubeAudioSourceManager, YoutubeSource, YoutubeSourceOptions, Client};
use crate::client::{WebClient, MusicClient, AndroidClient, WebEmbeddedClient, TvClient, IosClient};
use super::{YoutubeConfig, ClientProvider, ClientProviderV3, ClientProviderV4, PluginInfo};

/// YouTube plugin loader for Lavalink integration
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/YoutubePluginLoader.java`
/// 
/// This is the main entry point for the YouTube source plugin in Lavalink.
/// It handles configuration, client setup, and integration with the Lavalink audio player manager.
pub struct YoutubePluginLoader {
    config: Option<YoutubeConfig>,
    client_provider: Option<Box<dyn ClientProvider>>,
}

impl YoutubePluginLoader {
    /// Create a new plugin loader with configuration
    pub fn new(config: Option<YoutubeConfig>) -> Self {
        let client_provider = Self::create_client_provider();
        
        // Check for new releases (non-blocking)
        tokio::spawn(async {
            if let Err(e) = PluginInfo::check_for_new_release().await {
                debug!("Failed to check for new release: {e}");
            }
        });
        
        Self {
            config,
            client_provider,
        }
    }
    
    /// Create the appropriate client provider based on Lavalink version
    fn create_client_provider() -> Option<Box<dyn ClientProvider>> {
        if Self::is_v4_or_newer() {
            info!("Detected Lavalink v4+, using ClientProviderV4");
            Some(Box::new(ClientProviderV4::new()))
        } else {
            info!("Detected Lavalink v3, using ClientProviderV3");
            Some(Box::new(ClientProviderV3::new()))
        }
    }
    
    /// Check if we're running on Lavalink v4 or newer
    /// This is a simplified version - in a real implementation, this would check for specific Lavalink classes
    fn is_v4_or_newer() -> bool {
        // For now, assume v4+ since we're targeting modern Lavalink
        // In a real implementation, this would use reflection or feature detection
        true
    }
    
    /// Configure the YouTube audio source manager
    pub async fn configure_audio_source_manager(&self) -> crate::Result<YoutubeAudioSourceManager> {
        // Check if YouTube source is enabled
        if let Some(config) = &self.config {
            if !config.is_enabled() {
                warn!("YouTube source is disabled in configuration");
                return Err(crate::YoutubeError::ConfigurationError("YouTube source is disabled".to_string()));
            }
        }
        
        let source = self.create_source_manager().await?;
        self.configure_oauth(&source).await?;
        self.configure_po_token(&source)?;
        
        Ok(source)
    }
    
    /// Create the YouTube audio source manager with configured clients
    async fn create_source_manager(&self) -> crate::Result<YoutubeAudioSourceManager> {
        let options = self.create_source_options();
        
        if let Some(_provider) = &self.client_provider {
            let client_names = self.get_client_names();
            let clients = self.create_clients(&client_names).await?;
            
            info!("YouTube source initialised with clients: {}", 
                  client_names.join(", "));
            
            Ok(YoutubeAudioSourceManager::with_options_and_clients(options, clients))
        } else {
            warn!("ClientProvider instance is missing. The YouTube source will be initialised with default clients.");
            Ok(YoutubeAudioSourceManager::with_options(options))
        }
    }
    
    /// Create source options from configuration
    fn create_source_options(&self) -> YoutubeSourceOptions {
        if let Some(config) = &self.config {
            YoutubeSourceOptions::default()
                .set_allow_search(config.is_search_allowed())
                .set_allow_direct_video_ids(config.are_direct_video_ids_allowed())
                .set_allow_direct_playlist_ids(config.are_direct_playlist_ids_allowed())
        } else {
            YoutubeSourceOptions::default()
        }
    }
    
    /// Get the list of client names to use
    fn get_client_names(&self) -> Vec<String> {
        if let Some(config) = &self.config {
            config.get_clients()
        } else {
            warn!("youtubeConfig missing or 'clients' was not specified, default values will be used.");
            vec![
                "MUSIC".to_string(),
                "ANDROID_VR".to_string(),
                "WEB".to_string(),
                "WEBEMBEDDED".to_string(),
            ]
        }
    }
    
    /// Create client instances from client names
    async fn create_clients(&self, client_names: &[String]) -> crate::Result<Vec<Box<dyn Client>>> {
        let mut clients = Vec::new();
        
        for client_name in client_names {
            let client_options = self.config
                .as_ref()
                .map(|c| c.get_options_for_client(client_name))
                .unwrap_or_default();
                
            let client: Box<dyn Client> = match client_name.as_str() {
                "WEB" => Box::new(WebClient::new()?),
                "MUSIC" => Box::new(MusicClient::with_options(client_options)),
                "ANDROID" => Box::new(AndroidClient::with_options(client_options)),
                "ANDROID_VR" => Box::new(AndroidClient::vr_with_options(client_options)),
                "ANDROID_MUSIC" => Box::new(AndroidClient::music_with_options(client_options)),
                "WEBEMBEDDED" => Box::new(WebEmbeddedClient::with_options(client_options)),
                "IOS" => Box::new(IosClient::with_options(client_options)),
                "TV" => Box::new(TvClient::with_options(client_options)),
                _ => {
                    warn!("Unknown client type: {client_name}, skipping");
                    continue;
                }
            };
            
            clients.push(client);
        }
        
        if clients.is_empty() {
            return Err(crate::YoutubeError::ConfigurationError("No valid clients configured".to_string()));
        }
        
        Ok(clients)
    }
    
    /// Configure OAuth if enabled
    async fn configure_oauth(&self, _source: &YoutubeAudioSourceManager) -> crate::Result<()> {
        if let Some(config) = &self.config {
            if let Some(oauth_config) = &config.oauth {
                if oauth_config.is_enabled() {
                    debug!("Configuring youtube oauth integration with token: {:?} skipInitialization: {}", 
                           oauth_config.get_refresh_token().map(|_| "***"), 
                           oauth_config.should_skip_initialization());
                    
                    // TODO: Implement OAuth configuration
                    // source.use_oauth2(oauth_config.get_refresh_token().cloned(), oauth_config.should_skip_initialization()).await?;
                    warn!("OAuth configuration is not yet implemented in Rust version");
                }
            }
        }
        Ok(())
    }
    
    /// Configure PoToken if available
    fn configure_po_token(&self, _source: &YoutubeAudioSourceManager) -> crate::Result<()> {
        if let Some(config) = &self.config {
            if let Some(pot) = &config.pot {
                if let (Some(token), Some(visitor_data)) = (pot.get_token(), pot.get_visitor_data()) {
                    debug!("Applying poToken and visitorData to WEB & WEBEMBEDDED client (token: {token}, vd: {visitor_data})");
                    YoutubeSource::set_po_token_and_visitor_data(
                        Some(token.clone()), 
                        Some(visitor_data.clone())
                    );
                } else if pot.get_token().is_some() || pot.get_visitor_data().is_some() {
                    warn!("Both pot.token and pot.visitorData must be specified and valid for pot to apply.");
                }
            }
        }
        Ok(())
    }
    
    /// Get the configuration
    pub fn get_config(&self) -> Option<&YoutubeConfig> {
        self.config.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[tokio::test]
    async fn test_plugin_loader_creation() {
        let config = YoutubeConfig::new().set_enabled(true);
        let loader = YoutubePluginLoader::new(Some(config));
        
        assert!(loader.config.is_some());
        assert!(loader.client_provider.is_some());
    }
    
    #[tokio::test]
    async fn test_disabled_source() {
        let config = YoutubeConfig::new().set_enabled(false);
        let loader = YoutubePluginLoader::new(Some(config));
        
        let result = loader.configure_audio_source_manager().await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_client_names_default() {
        let loader = YoutubePluginLoader::new(None);
        let client_names = loader.get_client_names();

        assert_eq!(client_names, vec!["MUSIC", "ANDROID_VR", "WEB", "WEBEMBEDDED"]);
    }

    #[tokio::test]
    async fn test_client_names_custom() {
        let config = YoutubeConfig::new()
            .set_clients(vec!["WEB".to_string(), "MUSIC".to_string()]);
        let loader = YoutubePluginLoader::new(Some(config));
        let client_names = loader.get_client_names();

        assert_eq!(client_names, vec!["WEB", "MUSIC"]);
    }
}
