use url::Url;
use std::collections::HashMap;

/// Utility functions for URL parsing and manipulation
pub struct UrlTools;

impl UrlTools {
    /// Extract video ID from various YouTube URL formats
    pub fn extract_video_id(url: &str) -> Option<String> {
        if let Ok(parsed_url) = Url::parse(url) {
            // Handle different YouTube URL formats
            match parsed_url.host_str() {
                Some("www.youtube.com") | Some("youtube.com") => {
                    if parsed_url.path() == "/watch" {
                        // Standard watch URL: https://www.youtube.com/watch?v=VIDEO_ID
                        parsed_url.query_pairs()
                            .find(|(key, _)| key == "v")
                            .map(|(_, value)| value.to_string())
                    } else if parsed_url.path().starts_with("/embed/") {
                        // Embed URL: https://www.youtube.com/embed/VIDEO_ID
                        parsed_url.path()
                            .strip_prefix("/embed/")
                            .map(|id| id.to_string())
                    } else {
                        None
                    }
                }
                Some("youtu.be") => {
                    // Short URL: https://youtu.be/VIDEO_ID
                    parsed_url.path()
                        .strip_prefix("/")
                        .map(|id| id.to_string())
                }
                _ => None,
            }
        } else {
            // Maybe it's just a video ID
            if url.len() == 11 && url.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                Some(url.to_string())
            } else {
                None
            }
        }
    }

    /// Extract playlist ID from YouTube URL
    pub fn extract_playlist_id(url: &str) -> Option<String> {
        if let Ok(parsed_url) = Url::parse(url) {
            parsed_url.query_pairs()
                .find(|(key, _)| key == "list")
                .map(|(_, value)| value.to_string())
        } else {
            None
        }
    }

    /// Check if a string is a valid YouTube video ID format
    pub fn is_valid_video_id(id: &str) -> bool {
        id.len() == 11 && id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Check if a string is a valid YouTube playlist ID format
    pub fn is_valid_playlist_id(id: &str) -> bool {
        (id.starts_with("PL") || id.starts_with("UU") || id.starts_with("LL") ||
         id.starts_with("WL") || id.starts_with("RD") || id.starts_with("LM")) &&
        id.len() >= 10
    }

    /// Parse URL query parameters into a HashMap
    pub fn parse_query_params(url: &Url) -> HashMap<String, String> {
        url.query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// Extract all YouTube identifiers from a URL
    pub fn parse_youtube_url(url: &str) -> UrlInfo {
        if let Ok(parsed_url) = Url::parse(url) {
            let video_id = Self::extract_video_id(url);
            let playlist_id = Self::extract_playlist_id(url);
            let parameters = Self::parse_query_params(&parsed_url);

            UrlInfo {
                video_id,
                playlist_id,
                parameters,
                original_url: url.to_string(),
            }
        } else {
            // Try as direct ID
            if Self::is_valid_video_id(url) {
                UrlInfo {
                    video_id: Some(url.to_string()),
                    playlist_id: None,
                    parameters: HashMap::new(),
                    original_url: url.to_string(),
                }
            } else {
                UrlInfo {
                    video_id: None,
                    playlist_id: None,
                    parameters: HashMap::new(),
                    original_url: url.to_string(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UrlInfo {
    pub video_id: Option<String>,
    pub playlist_id: Option<String>,
    pub parameters: HashMap<String, String>,
    pub original_url: String,
}

/// Utility functions for working with YouTube API responses
pub struct JsonTools;

impl JsonTools {
    /// Safely navigate nested JSON structures
    pub fn navigate_json<'a>(
        json: &'a serde_json::Value,
        path: &[&str],
    ) -> Option<&'a serde_json::Value> {
        let mut current = json;
        for key in path {
            current = current.get(key)?;
        }
        Some(current)
    }

    /// Extract text from YouTube's text runs format
    pub fn extract_text_from_runs(runs: &serde_json::Value) -> Option<String> {
        if let Some(runs_array) = runs.as_array() {
            let text_parts: Vec<String> = runs_array
                .iter()
                .filter_map(|run| run.get("text")?.as_str())
                .map(|s| s.to_string())
                .collect();
            
            if text_parts.is_empty() {
                None
            } else {
                Some(text_parts.join(""))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            UrlTools::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        
        assert_eq!(
            UrlTools::extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        
        assert_eq!(
            UrlTools::extract_video_id("dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn test_extract_playlist_id() {
        assert_eq!(
            UrlTools::extract_playlist_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmRdnEQy4Qy9RBqOQQ1"),
            Some("PLrAXtmRdnEQy4Qy9RBqOQQ1".to_string())
        );
    }
}
