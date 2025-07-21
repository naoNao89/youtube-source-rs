use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeError {
    #[error("Cannot be loaded: {0}")]
    CannotBeLoaded(String),

    #[error("Option disabled: {0}")]
    OptionDisabled(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Video unavailable: {0}")]
    VideoUnavailable(String),

    #[error("Cipher error: {0}")]
    Cipher(String),

    #[error("JavaScript engine error: {0}")]
    JavaScriptEngine(#[from] crate::cipher::JavaScriptEngineError),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, YoutubeError>;

#[derive(Debug, Clone)]
pub enum AudioItem {
    Track(crate::YoutubeAudioTrack),
    Playlist(crate::YoutubePlaylist),
    SearchResult(crate::YoutubeSearchResult),
    NoMatches,
}
