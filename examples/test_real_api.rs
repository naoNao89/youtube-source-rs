use youtube_source_rs::client::WebClient;
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🧪 Testing Real YouTube API Integration");
    println!("=====================================");

    // Create a YouTube source manager with Web client
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true);

    let web_client = WebClient::new().expect("Failed to create WebClient");
    let clients: Vec<Box<dyn Client>> = vec![Box::new(web_client)];

    let manager = YoutubeAudioSourceManager::with_options_and_clients(options, clients);

    println!("✅ YouTube Source Manager created");

    // Test 1: Load a real video
    println!("\n🎬 Test 1: Loading real video (Rick Roll)");
    match manager.load_item("dQw4w9WgXcQ").await {
        Ok(Some(item)) => {
            println!("✅ Successfully loaded video!");
            match item {
                youtube_source_rs::AudioItem::Track(track) => {
                    println!("   📝 Title: {}", track.info.title);
                    println!("   👤 Author: {}", track.info.author);
                    println!("   ⏱️  Duration: {}s", track.info.duration.as_secs());
                    println!("   🆔 Video ID: {}", track.info.video_id);
                    println!("   🔗 URI: {}", track.info.uri);
                    if let Some(thumbnail) = &track.info.thumbnail {
                        println!("   🖼️  Thumbnail: {thumbnail}");
                    }
                }
                _ => println!("   ⚠️  Unexpected item type"),
            }
        }
        Ok(None) => println!("❌ No video found"),
        Err(e) => println!("❌ Error loading video: {e}"),
    }

    // Test 2: Search for videos
    println!("\n🔍 Test 2: Searching for 'rust programming'");
    match manager.load_item("ytsearch:rust programming").await {
        Ok(Some(item)) => {
            println!("✅ Successfully performed search!");
            match item {
                youtube_source_rs::AudioItem::SearchResult(search_result) => {
                    println!("   📊 Search query: {}", search_result.query);
                    println!("   📊 Total results: {}", search_result.tracks.len());

                    for (i, track) in search_result.tracks.iter().take(5).enumerate() {
                        println!(
                            "   {}. {} by {}",
                            i + 1,
                            track.info.title,
                            track.info.author
                        );
                    }
                }
                _ => println!("   ⚠️  Unexpected item type"),
            }
        }
        Ok(None) => println!("❌ No search results found"),
        Err(e) => println!("❌ Error performing search: {e}"),
    }

    // Test 3: Test invalid video
    println!("\n❌ Test 3: Testing invalid video ID");
    match manager.load_item("invalid_video_id").await {
        Ok(Some(_)) => println!("   ⚠️  Unexpected success with invalid video"),
        Ok(None) => println!("   ✅ Correctly returned None for invalid video"),
        Err(e) => println!("   ✅ Correctly failed with error: {e}"),
    }

    println!("\n🎉 API Integration Tests Complete!");

    Ok(())
}
