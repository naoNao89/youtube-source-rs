use crate::{Client, YoutubeSourceOptions, AudioItem, Result};

#[derive(Clone)]
pub struct YoutubeAudioSourceManager {
    pub options: YoutubeSourceOptions,
    pub clients: Vec<std::sync::Arc<dyn Client>>,
    pub http_client: reqwest::Client,
    // TODO: Add cipher manager, oauth handler, etc.
}

impl YoutubeAudioSourceManager {
    pub fn new() -> Self {
        Self::with_options(YoutubeSourceOptions::default())
    }

    pub fn with_options(options: YoutubeSourceOptions) -> Self {
        let clients: Vec<std::sync::Arc<dyn Client>> = vec![
            std::sync::Arc::new(crate::client::WebClient::new()),
            std::sync::Arc::new(crate::client::MusicClient::new()),
            std::sync::Arc::new(crate::client::AndroidClient::new()),
            std::sync::Arc::new(crate::client::WebEmbeddedClient::new()),
        ];

        Self {
            options,
            clients,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn load_item(&self, identifier: &str) -> Result<Option<AudioItem>> {
        // TODO: Implement router logic
        let router = self.get_router(identifier).await?;
        
        for client in &self.clients {
            if !client.can_handle_request(identifier) {
                continue;
            }

            log::debug!("Attempting to load {} with client \"{}\"", identifier, client.get_identifier());

            match router.route(client.as_ref(), self).await {
                Ok(Some(item)) => return Ok(Some(item)),
                Ok(None) => continue,
                Err(e) => {
                    log::debug!("Client {} failed: {}", client.get_identifier(), e);
                    continue;
                }
            }
        }

        Ok(None)
    }

    async fn get_router(&self, identifier: &str) -> Result<Router> {
        use crate::utils::UrlTools;

        // Check if it's a direct video ID
        if let Some(video_id) = UrlTools::extract_video_id(identifier) {
            // Check if it also has a playlist
            if let Some(playlist_id) = UrlTools::extract_playlist_id(identifier) {
                if playlist_id.starts_with("RD") {
                    return Ok(Router::Mix { mix_id: playlist_id, selected_video_id: Some(video_id) });
                } else if !playlist_id.starts_with("LL") && !playlist_id.starts_with("WL") && !playlist_id.starts_with("LM") {
                    return Ok(Router::Playlist { playlist_id, selected_video_id: Some(video_id) });
                }
            }
            return Ok(Router::Video { video_id });
        }

        // Check if it's a playlist URL
        if let Some(playlist_id) = UrlTools::extract_playlist_id(identifier) {
            if playlist_id.starts_with("RD") {
                return Ok(Router::Mix { mix_id: playlist_id, selected_video_id: None });
            } else if !playlist_id.starts_with("LL") && !playlist_id.starts_with("WL") && !playlist_id.starts_with("LM") {
                return Ok(Router::Playlist { playlist_id, selected_video_id: None });
            }
        }

        // Check if search is allowed
        if self.options.allow_search {
            return Ok(Router::Search { query: identifier.to_string() });
        }

        Ok(Router::None)
    }
}

enum Router {
    Video { video_id: String },
    Playlist { playlist_id: String, selected_video_id: Option<String> },
    Search { query: String },
    Mix { mix_id: String, selected_video_id: Option<String> },
    None,
}

impl Router {
    async fn route(&self, client: &dyn Client, source: &YoutubeAudioSourceManager) -> Result<Option<AudioItem>> {
        match self {
            Router::Video { video_id } => {
                client.load_video(source, video_id).await
            }
            Router::Playlist { playlist_id, selected_video_id } => {
                client.load_playlist(source, playlist_id, selected_video_id.as_deref()).await
            }
            Router::Search { query } => {
                client.search(source, query).await
            }
            Router::Mix { mix_id, selected_video_id } => {
                client.load_mix(source, mix_id, selected_video_id.as_deref()).await
            }
            Router::None => Ok(None),
        }
    }
}

impl std::fmt::Debug for YoutubeAudioSourceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YoutubeAudioSourceManager")
            .field("options", &self.options)
            .field("clients_count", &self.clients.len())
            .field("http_client", &"reqwest::Client")
            .finish()
    }
}
