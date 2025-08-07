//! YouTube Source Library for Rust
//!
//! A Rust implementation of the YouTube audio source manager
//! for Lavalink and similar audio streaming applications.

pub mod api;
pub mod cipher;
pub mod client;
pub mod config;
pub mod error;
pub mod http;
pub mod manager;
pub mod playlist;
pub mod plugin;
pub mod search;
pub mod track;
pub mod utils;

// Re-export main types
pub use client::{
    generate_capabilities_summary, AndroidClient, Client, ClientCapabilities, IosClient,
    MusicClient, TvClient, WebClient, WebEmbeddedClient,
};
pub use config::{ClientOptions, YoutubeSourceOptions};
pub use error::{AudioItem, Result, YoutubeError};
pub use manager::YoutubeAudioSourceManager;
pub use playlist::YoutubePlaylist;
pub use search::YoutubeSearchResult;
pub use track::{AudioTrackInfo, StreamFormat, TrackFormats, YoutubeAudioTrack};

// Re-export plugin types
pub use plugin::{
    ClientProvider, ClientProviderV3, ClientProviderV4, PluginInfo, Pot, YoutubeConfig,
    YoutubeOauthConfig, YoutubePluginLoader, YoutubeRestHandler,
};

/// Main entry point for the YouTube source library
pub struct YoutubeSource;

impl YoutubeSource {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Set PoToken and VisitorData for all supporting clients
    pub fn set_po_token_and_visitor_data(po_token: Option<String>, visitor_data: Option<String>) {
        // Implementation here
        log::debug!("Applying pot: {po_token:?} vd: {visitor_data:?} to WEB, WEBEMBEDDED");
    }
}
