use youtube_source_rs::client::{NonMusicClient, WebClient};
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🔍 Testing YouTube Search Functionality");
    println!("========================================");

    // Create a YouTube source manager with Web client
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true);

    let web_client = WebClient::new().expect("Failed to create WebClient");
    let clients: Vec<Box<dyn Client>> = vec![Box::new(web_client)];

    let manager = YoutubeAudioSourceManager::with_options_and_clients(options, clients);

    println!("✅ YouTube Source Manager created");

    // Test search queries
    let test_queries = vec![
        "Rick Astley Never Gonna Give You Up",
        "Beethoven Symphony 9",
        "lofi hip hop",
        "invalid query that should return nothing",
    ];

    for query in test_queries {
        println!("\n🔍 Searching for: \"{query}\"");

        // Get the first client to test search
        let client = &manager.clients[0];

        match client.search(&manager, query).await {
            Ok(Some(audio_item)) => {
                println!("✅ Search successful!");
                match audio_item {
                    youtube_source_rs::AudioItem::SearchResult(search_result) => {
                        println!(
                            "   📋 Search result with {} tracks and {} playlists",
                            search_result.tracks.len(),
                            search_result.playlists.len()
                        );

                        // Show first few tracks
                        for (i, track) in search_result.tracks.iter().enumerate().take(3) {
                            println!(
                                "     Track {}: {} by {}",
                                i + 1,
                                track.info.title,
                                track.info.author
                            );
                        }
                    }
                    youtube_source_rs::AudioItem::Track(track) => {
                        println!("   🎵 Found track: {}", track.info.title);
                        println!("   👤 Author: {}", track.info.author);
                        println!("   ⏱️  Duration: {}s", track.info.duration.as_secs());
                    }
                    youtube_source_rs::AudioItem::Playlist(playlist) => {
                        println!("   📋 Found playlist: {}", playlist.name);
                        println!("   📊 Track count: {}", playlist.tracks.len());
                    }
                    youtube_source_rs::AudioItem::NoMatches => {
                        println!("   ℹ️  No matches found for this query");
                    }
                }
            }
            Ok(None) => {
                println!("   ℹ️  No results found");
            }
            Err(e) => {
                println!("   ❌ Search failed: {e}");
            }
        }
    }

    // Test direct search results loading
    println!("\n🔍 Testing direct search results loading");
    let web_client = WebClient::new().expect("Failed to create WebClient");

    match web_client.load_search_results("Rick Astley").await {
        Ok(results) => {
            println!("✅ Direct search successful!");
            println!("   📊 Found {} SearchResult items", results.len());

            // Show basic info for SearchResult items
            for (i, result) in results.iter().enumerate().take(3) {
                println!("\n   Result {}:", i + 1);
                println!("     📄 Title: {}", result.title());
                println!("     👤 Author: {}", result.author());
                println!("     🏷️  Type: {}", result.result_type());

                // Show additional info based on result type
                match result {
                    youtube_source_rs::search::SearchResult::Video {
                        video_id, duration, ..
                    } => {
                        println!("     🎬 Video ID: {video_id}");
                        println!("     ⏱️  Duration: {}s", duration.as_secs());
                    }
                    youtube_source_rs::search::SearchResult::Playlist {
                        playlist_id,
                        video_count,
                        ..
                    } => {
                        println!("     📋 Playlist ID: {playlist_id}");
                        println!("     📊 Video count: {video_count}");
                    }
                    youtube_source_rs::search::SearchResult::Channel {
                        channel_id,
                        subscriber_count,
                        ..
                    } => {
                        println!("     📺 Channel ID: {channel_id}");
                        println!("     👥 Subscribers: {subscriber_count}");
                    }
                }
            }

            if results.len() > 3 {
                println!("   ... and {} more results", results.len() - 3);
            }
        }
        Err(e) => {
            println!("❌ Direct search failed: {e}");
        }
    }

    println!("\n🎉 Search tests complete!");

    Ok(())
}
