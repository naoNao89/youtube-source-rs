use youtube_source_rs::{
    utils::UrlTools, AudioItem, YoutubeAudioSourceManager, YoutubeSourceOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🎵 YouTube Source RS Demo");
    println!("========================");

    // Create a YouTube source manager with custom options
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true)
        .set_allow_direct_playlist_ids(true);

    let manager = YoutubeAudioSourceManager::with_options(options);

    println!("✅ YouTube Source Manager created successfully!");
    println!("📊 Available clients: {}", manager.clients.len());

    // Test URL parsing utilities
    println!("\n🔍 Testing URL parsing utilities:");

    let test_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "dQw4w9WgXcQ",
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmRdnEQy4Qy9RBqOQQ1",
    ];

    for url in test_urls {
        println!("  📎 URL: {url}");
        if let Some(video_id) = UrlTools::extract_video_id(url) {
            println!("    🎬 Video ID: {video_id}");
        }
        if let Some(playlist_id) = UrlTools::extract_playlist_id(url) {
            println!("    📋 Playlist ID: {playlist_id}");
        }

        let url_info = UrlTools::parse_youtube_url(url);
        println!("    📊 Parsed info: {url_info:?}");
        println!();
    }

    // Test loading items (with placeholder implementations)
    println!("🎵 Testing item loading:");

    let test_identifiers = vec![
        "dQw4w9WgXcQ",               // Direct video ID
        "rust programming tutorial", // Search query
    ];

    for identifier in test_identifiers {
        println!("  🔄 Loading: {identifier}");

        match manager.load_item(identifier).await {
            Ok(Some(item)) => match item {
                AudioItem::Track(track) => {
                    println!("    ✅ Found track: {}", track.info.title);
                    println!("       👤 Author: {}", track.info.author);
                    println!("       ⏱️  Duration: {:?}", track.info.duration);
                    println!("       🆔 Video ID: {}", track.info.video_id);
                }
                AudioItem::SearchResult(search) => {
                    println!("    ✅ Found search results for: {}", search.query);
                    println!("       📊 Total tracks: {}", search.tracks.len());
                    for (i, track) in search.tracks.iter().enumerate() {
                        println!(
                            "       {}. {} by {}",
                            i + 1,
                            track.info.title,
                            track.info.author
                        );
                    }
                }
                AudioItem::Playlist(playlist) => {
                    println!("    ✅ Found playlist: {}", playlist.name);
                    println!("       📊 Total tracks: {}", playlist.tracks.len());
                }
                AudioItem::NoMatches => {
                    println!("    ❌ No matches found");
                }
            },
            Ok(None) => {
                println!("    ❌ No result returned");
            }
            Err(e) => {
                println!("    ❌ Error: {e}");
            }
        }
        println!();
    }

    println!("🎉 Demo completed!");
    println!("\n📝 Note: This is a demonstration with placeholder implementations.");
    println!("   The actual YouTube API integration is still in development.");

    Ok(())
}
