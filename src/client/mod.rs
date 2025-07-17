pub mod traits;
pub mod config;
pub mod web;
pub mod music;
pub mod android;
pub mod embedded;

pub use traits::Client;
pub use config::ClientConfig;
pub use web::WebClient;
pub use music::MusicClient;
pub use android::AndroidClient;
pub use embedded::WebEmbeddedClient;
