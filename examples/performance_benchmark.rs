use std::time::Instant;
use youtube_source_rs::client::WebClient;
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 YouTube Source Rust Performance Benchmark");
    println!("============================================");

    // Create a YouTube source manager with Web client
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true);

    let web_client = WebClient::new().expect("Failed to create WebClient");
    let clients: Vec<Box<dyn Client>> = vec![Box::new(web_client)];

    let manager = YoutubeAudioSourceManager::with_options_and_clients(options, clients);

    println!("✅ YouTube Source Manager created");

    // Test videos for benchmarking
    let test_videos = vec![
        ("dQw4w9WgXcQ", "Rick Astley - Never Gonna Give You Up"),
        ("9bZkp7q19f0", "PSY - GANGNAM STYLE"),
        ("kJQP7kiw5Fk", "Luis Fonsi - Despacito"),
        ("fJ9rUzIMcZQ", "Queen - Bohemian Rhapsody"),
        ("YQHsXMglC9A", "Adele - Hello"),
    ];

    // Benchmark 1: Video Metadata Loading
    println!("\n📊 Benchmark 1: Video Metadata Loading");
    let start_time = Instant::now();
    let mut successful_loads = 0;

    for (video_id, title) in &test_videos {
        let video_start = Instant::now();

        match manager
            .load_item(&format!("https://www.youtube.com/watch?v={video_id}"))
            .await
        {
            Ok(Some(_)) => {
                let duration = video_start.elapsed();
                println!("   ✅ {title} loaded in {duration:?}");
                successful_loads += 1;
            }
            Ok(None) => {
                println!("   ❌ {title} - No video found");
            }
            Err(e) => {
                println!("   ❌ {title} - Error: {e}");
            }
        }
    }

    let total_metadata_time = start_time.elapsed();
    println!("   📈 Total metadata loading: {total_metadata_time:?}");
    println!(
        "   📈 Average per video: {:?}",
        total_metadata_time / test_videos.len() as u32
    );
    println!(
        "   📈 Success rate: {}/{}",
        successful_loads,
        test_videos.len()
    );

    // Benchmark 2: Stream Format Extraction
    println!("\n📊 Benchmark 2: Stream Format Extraction");
    let start_time = Instant::now();
    let mut successful_formats = 0;
    let mut total_formats = 0;

    for (video_id, title) in &test_videos {
        let format_start = Instant::now();
        let client = &manager.clients[0];

        match client.get_track_formats(&manager, video_id).await {
            Ok(formats) => {
                let duration = format_start.elapsed();
                println!(
                    "   ✅ {} - {} formats in {:?}",
                    title,
                    formats.formats.len(),
                    duration
                );
                successful_formats += 1;
                total_formats += formats.formats.len();
            }
            Err(e) => {
                println!("   ❌ {title} - Error: {e}");
            }
        }
    }

    let total_format_time = start_time.elapsed();
    println!("   📈 Total format extraction: {total_format_time:?}");
    println!(
        "   📈 Average per video: {:?}",
        total_format_time / test_videos.len() as u32
    );
    println!(
        "   📈 Success rate: {}/{}",
        successful_formats,
        test_videos.len()
    );
    println!("   📈 Total formats extracted: {total_formats}");

    // Benchmark 3: Search Performance
    println!("\n📊 Benchmark 3: Search Performance");
    let search_queries = vec![
        "rust programming",
        "music 2024",
        "funny videos",
        "tutorial programming",
        "relaxing music",
    ];

    let start_time = Instant::now();
    let mut successful_searches = 0;
    let mut total_results = 0;

    for query in &search_queries {
        let search_start = Instant::now();
        let client = &manager.clients[0];

        match client.search(&manager, query).await {
            Ok(Some(audio_item)) => {
                let duration = search_start.elapsed();
                match audio_item {
                    youtube_source_rs::AudioItem::SearchResult(search_result) => {
                        let result_count = search_result.tracks.len();
                        println!("   ✅ '{query}' - {result_count} results in {duration:?}");
                        successful_searches += 1;
                        total_results += result_count;
                    }
                    _ => {
                        println!("   ⚠️  '{query}' - Unexpected result type");
                    }
                }
            }
            Ok(None) => {
                println!("   ❌ '{query}' - No results");
            }
            Err(e) => {
                println!("   ❌ '{query}' - Error: {e}");
            }
        }
    }

    let total_search_time = start_time.elapsed();
    println!("   📈 Total search time: {total_search_time:?}");
    println!(
        "   📈 Average per search: {:?}",
        total_search_time / search_queries.len() as u32
    );
    println!(
        "   📈 Success rate: {}/{}",
        successful_searches,
        search_queries.len()
    );
    println!("   📈 Total results found: {total_results}");

    // Benchmark 4: Playlist Loading
    println!("\n📊 Benchmark 4: Playlist Loading");
    let test_playlists = vec![
        ("PLFgquLnL59alCl_2TQvOiD5Vgm1hCaGSI", "Popular Music Videos"),
        ("PLrAXtmRdnEQy8VEwtaZnWxaHSUXoPXDk5", "Test Playlist"),
    ];

    let start_time = Instant::now();
    let mut successful_playlists = 0;
    let mut total_tracks = 0;

    for (playlist_id, name) in &test_playlists {
        let playlist_start = Instant::now();

        match manager
            .load_item(&format!(
                "https://www.youtube.com/playlist?list={playlist_id}"
            ))
            .await
        {
            Ok(Some(item)) => {
                let duration = playlist_start.elapsed();
                match item {
                    youtube_source_rs::AudioItem::Playlist(playlist) => {
                        println!(
                            "   ✅ {} - {} tracks in {:?}",
                            name,
                            playlist.tracks.len(),
                            duration
                        );
                        successful_playlists += 1;
                        total_tracks += playlist.tracks.len();
                    }
                    _ => {
                        println!("   ⚠️  {name} - Unexpected item type");
                    }
                }
            }
            Ok(None) => {
                println!("   ❌ {name} - No playlist found");
            }
            Err(e) => {
                println!("   ❌ {name} - Error: {e}");
            }
        }
    }

    let total_playlist_time = start_time.elapsed();
    println!("   📈 Total playlist loading: {total_playlist_time:?}");
    println!(
        "   📈 Average per playlist: {:?}",
        total_playlist_time / test_playlists.len() as u32
    );
    println!(
        "   📈 Success rate: {}/{}",
        successful_playlists,
        test_playlists.len()
    );
    println!("   📈 Total tracks loaded: {total_tracks}");

    // Summary
    println!("\n🎉 Performance Benchmark Complete!");
    println!("==================================");
    println!("📊 Summary:");
    println!("   🎵 Video metadata: {successful_loads} successful loads");
    println!("   🎧 Format extraction: {successful_formats} successful extractions");
    println!("   🔍 Search: {successful_searches} successful searches");
    println!("   📋 Playlists: {successful_playlists} successful loads");

    let total_benchmark_time =
        total_metadata_time + total_format_time + total_search_time + total_playlist_time;
    println!("   ⏱️  Total benchmark time: {total_benchmark_time:?}");

    // Performance targets (based on PROGRESS.md goals)
    println!("\n🎯 Performance Targets:");
    println!("   🚀 Startup Time: Target 50% faster than Java");
    println!("   💾 Memory Usage: Target 60% reduction");
    println!("   📡 Request Latency: Target 30% improvement");
    println!("   🔄 Concurrent Requests: Target 2x better handling");

    Ok(())
}
