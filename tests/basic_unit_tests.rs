use youtube_source_rs::client::{AndroidClient, IosClient, MusicClient, TvClient, WebClient};
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[test]
fn test_android_client_creation() {
    let client = AndroidClient::new();
    assert_eq!(client.get_identifier(), "ANDROID");
}

#[test]
fn test_web_client_creation() {
    let client = WebClient::new().expect("Failed to create WebClient");
    assert_eq!(client.get_identifier(), "WEB");
}

#[test]
fn test_music_client_creation() {
    let client = MusicClient::new();
    assert_eq!(client.get_identifier(), "MUSIC");
}

#[test]
fn test_ios_client_creation() {
    let client = IosClient::new();
    assert_eq!(client.get_identifier(), "IOS");
}

#[test]
fn test_tv_client_creation() {
    let client = TvClient::new();
    assert_eq!(client.get_identifier(), "TVHTML5");
}

#[test]
fn test_client_can_handle_video() {
    let client = AndroidClient::new();
    assert!(client.can_handle_request("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    assert!(client.can_handle_request("https://youtu.be/dQw4w9WgXcQ"));
    // AndroidClient (Standard variant) accepts all requests by design
    assert!(client.can_handle_request("https://example.com"));
}

#[test]
fn test_client_can_handle_playlist() {
    let client = AndroidClient::new();
    assert!(client.can_handle_request("https://www.youtube.com/playlist?list=PLtest"));
    // AndroidClient (Standard variant) accepts all requests by design
    assert!(client.can_handle_request("https://example.com/playlist"));
}

#[test]
fn test_client_can_handle_search() {
    let client = AndroidClient::new();
    assert!(client.can_handle_request("ytsearch:test query"));
    assert!(client.can_handle_request("ytmsearch:music query"));
    // AndroidClient (Standard variant) accepts all requests by design
    assert!(client.can_handle_request("search:invalid"));
}

#[test]
fn test_manager_creation() {
    let options = YoutubeSourceOptions::default();
    let manager = YoutubeAudioSourceManager::with_options(options);

    // Manager should be created successfully
    assert!(!manager.clients.is_empty());
}

#[test]
fn test_manager_with_custom_clients() {
    let options = YoutubeSourceOptions::default();
    let clients: Vec<Box<dyn Client>> =
        vec![Box::new(AndroidClient::new()), Box::new(MusicClient::new())];

    let manager = YoutubeAudioSourceManager::with_options_and_clients(options, clients);
    assert_eq!(manager.clients.len(), 2);
}

#[test]
fn test_client_capabilities() {
    let android_client = AndroidClient::new();
    let capabilities = android_client.get_capabilities();

    // Android client should support videos
    assert!(capabilities.videos);
}

#[test]
fn test_source_options_default() {
    let options = YoutubeSourceOptions::default();

    // Default options should be valid
    assert!(options.allow_search);
    assert!(options.allow_direct_video_ids);
    assert!(options.allow_direct_playlist_ids);
}
