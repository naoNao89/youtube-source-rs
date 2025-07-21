use youtube_source_rs::client::WebClient;
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🎵 Testing Stream Format Loading");
    println!("================================");

    // Create a YouTube source manager with Web client
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true);

    let web_client = WebClient::new().expect("Failed to create WebClient");
    let clients: Vec<Box<dyn Client>> = vec![Box::new(web_client)];

    let manager = YoutubeAudioSourceManager::with_options_and_clients(options, clients);

    println!("✅ YouTube Source Manager created");

    // Test format loading for Rick Roll
    let video_id = "dQw4w9WgXcQ";
    println!("\n🎬 Loading formats for video: {video_id}");

    // Get the first client to test format loading
    let client = &manager.clients[0];

    match client.get_track_formats(&manager, video_id).await {
        Ok(track_formats) => {
            println!("✅ Successfully loaded track formats!");
            println!("   📊 Total formats: {}", track_formats.formats.len());
            println!(
                "   🔗 Player script URL: {}",
                track_formats.player_script_url
            );

            // Show format details
            for (i, format) in track_formats.formats.iter().enumerate().take(10) {
                println!("\n   Format {}:", i + 1);
                println!("     🏷️  itag: {}", format.itag);
                println!("     📄 Content type: {}", format.content_type);
                println!("     🎵 Bitrate: {} kbps", format.bitrate / 1000);
                println!("     📏 Content length: {} bytes", format.content_length);
                println!("     🔊 Audio channels: {}", format.audio_channels);
                println!("     🎯 Default audio: {}", format.is_default_audio_track);
                println!("     🔐 Has signature: {}", format.signature.is_some());
                println!(
                    "     🔗 URL: {}...",
                    &format.url.to_string()[..format.url.to_string().len().min(80)]
                );

                if let Some(info) = &format.info {
                    println!("     📋 Format info: {info:?}");
                    println!("     🎭 MIME type: {}", info.mime_type());
                    println!("     🎼 Codec: {}", info.codec());
                }
            }

            // Test getting best format
            println!("\n🏆 Testing best format selection:");
            match track_formats.get_best_format() {
                Ok(best_format) => {
                    println!("   ✅ Best format found!");
                    println!("     🏷️  itag: {}", best_format.itag);
                    println!("     📄 Content type: {}", best_format.content_type);
                    println!("     🎵 Bitrate: {} kbps", best_format.bitrate / 1000);
                    println!("     🔊 Audio channels: {}", best_format.audio_channels);
                    if let Some(info) = &best_format.info {
                        println!("     📋 Format: {:?} ({})", info, info.codec());
                    }
                }
                Err(e) => {
                    println!("   ❌ Failed to get best format: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load track formats: {e}");
        }
    }

    // Test with different video types
    let test_videos = vec![
        ("jNQXAC9IVRw", "Me at the zoo - First YouTube video"),
        ("9bZkp7q19f0", "PSY - Gangnam Style"),
    ];

    for (video_id, description) in test_videos {
        println!("\n🎬 Testing formats for: {description} ({video_id})");

        match client.get_track_formats(&manager, video_id).await {
            Ok(track_formats) => {
                println!(
                    "   ✅ Success: {} formats found",
                    track_formats.formats.len()
                );

                // Count audio vs video formats
                let audio_count = track_formats
                    .formats
                    .iter()
                    .filter(|f| f.content_type.starts_with("audio/"))
                    .count();
                let video_count = track_formats.formats.len() - audio_count;

                println!("   🎵 Audio formats: {audio_count}");
                println!("   🎬 Video formats: {video_count}");

                // Show best audio format
                if let Ok(best) = track_formats.get_best_format() {
                    println!(
                        "   🏆 Best: {} @ {} kbps",
                        best.content_type,
                        best.bitrate / 1000
                    );
                }
            }
            Err(e) => {
                println!("   ❌ Failed: {e}");
            }
        }
    }

    println!("\n🎉 Format loading tests complete!");

    Ok(())
}
