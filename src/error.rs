use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeError {
    #[error("Cannot be loaded: {0}")]
    CannotBeLoaded(String),
    
    #[error("Option disabled: {0}")]
    OptionDisabled(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Cipher error: {0}")]
    Cipher(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Rate limited")]
    RateLimit,
    
    #[error("Video unavailable: {0}")]
    VideoUnavailable(String),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, YoutubeError>;

#[derive(Debug, Clone)]
pub enum AudioItem {
    Track(crate::YoutubeAudioTrack),
    Playlist(crate::YoutubePlaylist),
    SearchResult(crate::YoutubeSearchResult),
    NoMatches,
}
