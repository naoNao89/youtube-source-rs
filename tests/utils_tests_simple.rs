use pretty_assertions::assert_eq;

#[test]
fn test_extract_video_id() {
    let test_cases = vec![
        (
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            Some("dQw4w9WgXcQ"),
        ),
        ("https://youtu.be/dQw4w9WgXcQ", Some("dQw4w9WgXcQ")),
        (
            "https://youtube.com/watch?v=dQw4w9WgXcQ",
            Some("dQw4w9WgXcQ"),
        ),
        (
            "https://m.youtube.com/watch?v=dQw4w9WgXcQ",
            Some("dQw4w9WgXcQ"),
        ),
        (
            "https://www.youtube.com/embed/dQw4w9WgXcQ",
            Some("dQw4w9WgXcQ"),
        ),
        ("https://www.youtube.com/v/dQw4w9WgXcQ", Some("dQw4w9WgXcQ")),
        ("dQw4w9WgXcQ", Some("dQw4w9WgXcQ")),
        ("https://example.com", None),
        ("invalid", None),
        ("", None),
    ];

    for (input, expected) in test_cases {
        // Simple regex-based extraction for testing
        let result = extract_video_id(input);
        assert_eq!(
            result,
            expected.map(|s| s.to_string()),
            "Failed for input: {}",
            input
        );
    }
}

#[test]
fn test_extract_playlist_id() {
    let test_cases = vec![
        (
            "https://www.youtube.com/playlist?list=PLtest123",
            Some("PLtest123"),
        ),
        (
            "https://youtube.com/playlist?list=PLtest123",
            Some("PLtest123"),
        ),
        (
            "https://m.youtube.com/playlist?list=PLtest123",
            Some("PLtest123"),
        ),
        ("PLtest123", Some("PLtest123")),
        (
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLtest123",
            Some("PLtest123"),
        ),
        ("https://example.com", None),
        ("invalid", None),
        ("", None),
    ];

    for (input, expected) in test_cases {
        // Simple regex-based extraction for testing
        let result = extract_playlist_id(input);
        assert_eq!(
            result,
            expected.map(|s| s.to_string()),
            "Failed for input: {}",
            input
        );
    }
}

#[test]
fn test_url_parsing() {
    let valid_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "https://youtube.com/watch?v=dQw4w9WgXcQ",
        "https://m.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://music.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://www.youtube.com/playlist?list=PLtest",
        "https://www.youtube.com/embed/dQw4w9WgXcQ",
    ];

    let invalid_urls = vec![
        "https://example.com",
        "https://vimeo.com/123456",
        "https://soundcloud.com/track",
        "not a url",
        "",
    ];

    for url in valid_urls {
        assert!(is_youtube_url(url), "Should be valid YouTube URL: {url}");
    }

    for url in invalid_urls {
        assert!(
            !is_youtube_url(url),
            "Should not be valid YouTube URL: {url}"
        );
    }
}

#[test]
fn test_search_query() {
    let search_queries = vec![
        "ytsearch:never gonna give you up",
        "ytmsearch:rick astley",
        "ytsearch:\"exact phrase\"",
        "ytmsearch:artist - song title",
    ];

    let non_search_queries = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "dQw4w9WgXcQ",
        "just some text",
        "",
    ];

    for query in search_queries {
        assert!(is_search_query(query), "Should be search query: {query}");
    }

    for query in non_search_queries {
        assert!(
            !is_search_query(query),
            "Should not be search query: {query}"
        );
    }
}

// Helper functions for testing

fn extract_video_id(input: &str) -> Option<String> {
    let video_id_regex = regex::Regex::new(
        r"(?:youtube\.com/(?:watch\?v=|v/|embed/)|youtu\.be/)([a-zA-Z0-9_-]{11})",
    )
    .unwrap();

    if let Some(captures) = video_id_regex.captures(input) {
        if let Some(id) = captures.get(1) {
            return Some(id.as_str().to_string());
        }
    }

    // Direct video ID
    if input.len() == 11
        && input
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Some(input.to_string());
    }

    None
}

fn extract_playlist_id(input: &str) -> Option<String> {
    let playlist_regex = regex::Regex::new(r"(?:youtube\.com/.*?list=)([a-zA-Z0-9_-]+)").unwrap();

    if let Some(captures) = playlist_regex.captures(input) {
        if let Some(id) = captures.get(1) {
            return Some(id.as_str().to_string());
        }
    }

    // Direct playlist ID
    if input.starts_with("PL") && input.len() > 2 {
        return Some(input.to_string());
    }

    None
}

fn is_youtube_url(url: &str) -> bool {
    let youtube_domains = ["youtube.com", "youtu.be", "music.youtube.com"];

    for domain in youtube_domains {
        if url.contains(domain) {
            return true;
        }
    }

    false
}

fn is_search_query(query: &str) -> bool {
    query.starts_with("ytsearch:") || query.starts_with("ytmsearch:")
}
