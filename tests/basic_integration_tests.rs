use youtube_source_rs::client::AndroidClient;
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::test]
async fn test_manager_creation() {
    let options = YoutubeSourceOptions::default();
    let manager = YoutubeAudioSourceManager::with_options(options);

    // Manager should be created successfully
    assert!(!manager.clients.is_empty());
    assert!(manager.options.allow_search);
    assert!(manager.options.allow_direct_video_ids);
    assert!(manager.options.allow_direct_playlist_ids);
}

#[tokio::test]
async fn test_client_request_handling() {
    let client = AndroidClient::new();

    // Test various YouTube URL formats
    assert!(client.can_handle_request("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    assert!(client.can_handle_request("https://youtu.be/dQw4w9WgXcQ"));
    assert!(client.can_handle_request("https://m.youtube.com/watch?v=dQw4w9WgXcQ"));
    assert!(client.can_handle_request("https://music.youtube.com/watch?v=dQw4w9WgXcQ"));

    // Test playlist URLs
    assert!(client.can_handle_request("https://www.youtube.com/playlist?list=PLtest123"));

    // Test search queries
    assert!(client.can_handle_request("ytsearch:never gonna give you up"));
    assert!(client.can_handle_request("ytmsearch:rick astley"));

    // Note: AndroidClient (Standard variant) accepts all requests by design
    // Other clients like WebClient have more specific validation
    assert!(client.can_handle_request("https://example.com")); // AndroidClient accepts all
    assert!(client.can_handle_request("invalid://url")); // AndroidClient accepts all
    assert!(client.can_handle_request("")); // AndroidClient accepts all
}

#[test]
fn test_client_capabilities_consistency() {
    let clients: Vec<Box<dyn Client>> = vec![
        Box::new(AndroidClient::new()),
        Box::new(youtube_source_rs::client::WebClient::new().unwrap()),
        Box::new(youtube_source_rs::client::MusicClient::new()),
        Box::new(youtube_source_rs::client::IosClient::new()),
        Box::new(youtube_source_rs::client::TvClient::new()),
    ];

    for client in clients {
        let capabilities = client.get_capabilities();

        // All clients should have some basic capabilities
        assert!(
            capabilities.videos || capabilities.playlists || capabilities.search,
            "Client {} should support at least one capability",
            client.get_identifier()
        );
    }
}

#[test]
fn test_url_validation() {
    let client = AndroidClient::new();

    // Valid YouTube URLs
    let valid_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "https://www.youtube.com/playlist?list=PLtest",
        "ytsearch:test query",
        "ytmsearch:music query",
    ];

    for url in valid_urls {
        assert!(
            client.can_handle_request(url),
            "Should handle valid URL: {url}"
        );
    }

    // Test with WebClient which has stricter validation
    let web_client = youtube_source_rs::client::WebClient::new().unwrap();

    // Invalid URLs for WebClient (which has proper URL validation)
    let invalid_urls = vec![
        "https://example.com",
        "https://vimeo.com/123456",
        "invalid://url",
        "",
        "just text",
    ];

    for url in invalid_urls {
        assert!(
            !web_client.can_handle_request(url),
            "Should not handle invalid URL: {url}"
        );
    }
}

#[test]
fn test_manager_client_selection() {
    let options = YoutubeSourceOptions::default();
    let manager = YoutubeAudioSourceManager::with_options(options);

    // Manager should have multiple clients available
    assert!(
        !manager.clients.is_empty(),
        "Manager should have at least one client"
    );

    // All clients should be properly initialized
    for client in &manager.clients {
        assert!(
            !client.get_identifier().is_empty(),
            "Client identifier should not be empty"
        );
    }
}
