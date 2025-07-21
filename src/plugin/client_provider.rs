use std::collections::HashMap;
use log::{debug, warn};
use crate::{Client, ClientOptions};
use crate::client::{
    WebClient, MusicClient, AndroidClient, WebEmbeddedClient, 
    IosClient, TvClient
};

/// Options provider trait for client configuration
pub trait OptionsProvider {
    fn get_options_for_client(&self, client_name: &str) -> ClientOptions;
}

/// Client reference for dynamic client creation
pub struct ClientReference {
    pub name: String,
    pub factory: Box<dyn Fn(ClientOptions) -> crate::Result<Box<dyn Client>> + Send + Sync>,
}

impl ClientReference {
    pub fn new<F>(name: &str, factory: F) -> Self 
    where 
        F: Fn(ClientOptions) -> crate::Result<Box<dyn Client>> + Send + Sync + 'static
    {
        Self {
            name: name.to_string(),
            factory: Box::new(factory),
        }
    }
    
    pub fn get_client(&self, options: ClientOptions) -> crate::Result<Box<dyn Client>> {
        debug!("Initialising client {} with options {:?}", self.name, options);
        (self.factory)(options)
    }
}

/// Base client provider trait
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/ClientProvider.java`
pub trait ClientProvider: Send + Sync {
    /// Get the default client names
    fn get_default_clients(&self) -> Vec<String> {
        // This is a default list of clients. This list matches that of the
        // YoutubeAudioSourceManager. If that is updated, this should probably be
        // updated too.
        vec![
            "MUSIC".to_string(),
            "WEB".to_string(), 
            "ANDROID_VR".to_string(),
            "WEBEMBEDDED".to_string(),
        ]
    }
    
    /// Get clients by name with options provider
    fn get_clients(&self, client_names: &[String], options_provider: &dyn OptionsProvider) -> crate::Result<Vec<Box<dyn Client>>>;
    
    /// Get available client references
    fn get_client_references(&self) -> Vec<ClientReference>;
    
    /// Get a client by name using the available references
    fn get_client_by_name(&self, name: &str, options_provider: &dyn OptionsProvider) -> Option<Box<dyn Client>> {
        let references = self.get_client_references();
        
        for reference in references {
            if reference.name == name {
                let options = options_provider.get_options_for_client(name);
                match reference.get_client(options) {
                    Ok(client) => return Some(client),
                    Err(e) => {
                        warn!("Failed to create client {name}: {e}");
                        return None;
                    }
                }
            }
        }
        
        warn!("Failed to resolve {name} into a Client");
        None
    }
}

/// Client provider for Lavalink v3
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/ClientProviderV3.java`
pub struct ClientProviderV3;

impl ClientProviderV3 {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClientProviderV3 {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientProvider for ClientProviderV3 {
    fn get_clients(&self, client_names: &[String], options_provider: &dyn OptionsProvider) -> crate::Result<Vec<Box<dyn Client>>> {
        let mut resolved = Vec::new();
        
        for client_name in client_names {
            if let Some(client) = self.get_client_by_name(client_name, options_provider) {
                resolved.push(client);
            }
        }
        
        if resolved.is_empty() {
            return Err(crate::YoutubeError::ConfigurationError("No valid clients could be created".to_string()));
        }
        
        Ok(resolved)
    }
    
    fn get_client_references(&self) -> Vec<ClientReference> {
        // We can't clone the closures, so we recreate them
        vec![
            ClientReference::new("MUSIC", |opts| Ok(Box::new(MusicClient::with_options(opts)))),
            ClientReference::new("WEB", |_opts| Ok(Box::new(WebClient::new()?))),
            ClientReference::new("WEBEMBEDDED", |opts| Ok(Box::new(WebEmbeddedClient::with_options(opts)))),
            ClientReference::new("ANDROID", |opts| Ok(Box::new(AndroidClient::with_options(opts)))),
            ClientReference::new("ANDROID_VR", |opts| Ok(Box::new(AndroidClient::vr_with_options(opts)))),
            ClientReference::new("ANDROID_MUSIC", |opts| Ok(Box::new(AndroidClient::music_with_options(opts)))),
            ClientReference::new("IOS", |opts| Ok(Box::new(IosClient::with_options(opts)))),
            ClientReference::new("TV", |opts| Ok(Box::new(TvClient::with_options(opts)))),
            ClientReference::new("TVHTML5EMBEDDED", |opts| Ok(Box::new(TvClient::html5_embedded_with_options(opts)))),
        ]
    }
}

/// Client provider for Lavalink v4
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/ClientProviderV4.java`
pub struct ClientProviderV4;

impl ClientProviderV4 {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClientProviderV4 {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientProvider for ClientProviderV4 {
    fn get_clients(&self, client_names: &[String], options_provider: &dyn OptionsProvider) -> crate::Result<Vec<Box<dyn Client>>> {
        let mut resolved = Vec::new();
        
        for client_name in client_names {
            if let Some(client) = self.get_client_by_name(client_name, options_provider) {
                resolved.push(client);
            }
        }
        
        if resolved.is_empty() {
            return Err(crate::YoutubeError::ConfigurationError("No valid clients could be created".to_string()));
        }
        
        Ok(resolved)
    }
    
    fn get_client_references(&self) -> Vec<ClientReference> {
        // We can't clone the closures, so we recreate them
        vec![
            ClientReference::new("MUSIC", |opts| Ok(Box::new(MusicClient::with_options(opts)))),
            ClientReference::new("WEB", |_opts| Ok(Box::new(WebClient::new()?))),
            ClientReference::new("WEBEMBEDDED", |opts| Ok(Box::new(WebEmbeddedClient::with_options(opts)))),
            ClientReference::new("ANDROID", |opts| Ok(Box::new(AndroidClient::with_options(opts)))),
            ClientReference::new("ANDROID_VR", |opts| Ok(Box::new(AndroidClient::vr_with_options(opts)))),
            ClientReference::new("ANDROID_MUSIC", |opts| Ok(Box::new(AndroidClient::music_with_options(opts)))),
            ClientReference::new("IOS", |opts| Ok(Box::new(IosClient::with_options(opts)))),
            ClientReference::new("TV", |opts| Ok(Box::new(TvClient::with_options(opts)))),
            ClientReference::new("TVHTML5EMBEDDED", |opts| Ok(Box::new(TvClient::html5_embedded_with_options(opts)))),
        ]
    }
}

/// Simple options provider implementation
pub struct SimpleOptionsProvider {
    options: HashMap<String, ClientOptions>,
}

impl SimpleOptionsProvider {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }
    
    pub fn add_options(&mut self, client_name: String, options: ClientOptions) {
        self.options.insert(client_name, options);
    }
}

impl Default for SimpleOptionsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionsProvider for SimpleOptionsProvider {
    fn get_options_for_client(&self, client_name: &str) -> ClientOptions {
        self.options.get(client_name).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_provider_v3_default_clients() {
        let provider = ClientProviderV3::new();
        let default_clients = provider.get_default_clients();
        
        assert_eq!(default_clients, vec!["MUSIC", "WEB", "ANDROID_VR", "WEBEMBEDDED"]);
    }
    
    #[test]
    fn test_client_provider_v4_default_clients() {
        let provider = ClientProviderV4::new();
        let default_clients = provider.get_default_clients();
        
        assert_eq!(default_clients, vec!["MUSIC", "WEB", "ANDROID_VR", "WEBEMBEDDED"]);
    }
    
    #[test]
    fn test_simple_options_provider() {
        let mut provider = SimpleOptionsProvider::new();
        let options = ClientOptions::default();
        provider.add_options("WEB".to_string(), options.clone());
        
        assert_eq!(provider.get_options_for_client("WEB"), options);
        assert_eq!(provider.get_options_for_client("NONEXISTENT"), ClientOptions::default());
    }
}
