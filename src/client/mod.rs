pub mod android;
pub mod base;
pub mod config;
pub mod embedded;
pub mod ios;
pub mod music;
pub mod traits;
pub mod tv;
pub mod web;

pub use android::{AndroidClient, AndroidVariant};
pub use base::{NonMusicClient, NonMusicClientBase, PlayabilityStatus};
pub use config::ClientConfig;
pub use embedded::WebEmbeddedClient;
pub use ios::IosClient;
pub use music::MusicClient;
pub use traits::{generate_capabilities_summary, Client, ClientCapabilities};
pub use tv::{TvClient, TvVariant};
pub use web::{WebClient, WebVariant};
