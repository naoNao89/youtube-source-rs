use youtube_source_rs::{YoutubeAudioSourceManager, YoutubeSourceOptions, utils::UrlTools};

#[tokio::test]
async fn test_manager_creation() {
    let manager = YoutubeAudioSourceManager::new();
    assert_eq!(manager.clients.len(), 4); // Music, Android, Web, WebEmbedded
}

#[tokio::test]
async fn test_manager_with_options() {
    let options = YoutubeSourceOptions::default()
        .set_allow_search(false)
        .set_allow_direct_video_ids(true);

    let manager = YoutubeAudioSourceManager::with_options(options);
    assert!(!manager.options.allow_search);
    assert!(manager.options.allow_direct_video_ids);
}

#[test]
fn test_url_tools() {
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
