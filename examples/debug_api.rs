use youtube_source_rs::client::{NonMusicClient, WebClient};
use youtube_source_rs::http::RequestContext;
use youtube_source_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    println!("🔍 Debug YouTube API Integration");
    println!("=================================");

    // Create a Web client directly for testing
    let web_client = WebClient::new().expect("Failed to create WebClient");

    println!("✅ WebClient created successfully");
    println!("   Client ID: {}", web_client.get_identifier());
    println!("   Config: {:?}", web_client.get_client_config());

    // Test 1: Direct API call to load track info
    println!("\n🎬 Test 1: Direct track info loading");
    let video_id = "dQw4w9WgXcQ"; // Rick Roll
    println!("   Video ID: {video_id}");

    match web_client.load_track_info_from_innertube(video_id).await {
        Ok(track_info) => {
            println!("✅ Successfully loaded track info!");
            println!("   Title: {}", track_info.title);
            println!("   Author: {}", track_info.author);
            println!("   Duration: {}s", track_info.duration.as_secs());
            println!("   Video ID: {}", track_info.video_id);
            println!("   URI: {}", track_info.uri);
            println!("   Is Live: {}", track_info.is_stream);
        }
        Err(e) => {
            println!("❌ Failed to load track info: {e}");
            println!("   Error type: {e:?}");
        }
    }

    // Test 2: Manual HTTP request to YouTube API
    println!("\n🌐 Test 2: Manual HTTP request");
    let http_client = web_client.get_http_client();
    let client_config = web_client.get_client_config();

    // Build request payload manually
    let payload = serde_json::json!({
        "context": client_config.to_context_json(),
        "videoId": video_id
    });

    println!(
        "   Request payload: {}",
        serde_json::to_string_pretty(&payload)?
    );

    let api_key = client_config.get_api_key().unwrap_or("NO_API_KEY");
    let url = format!("https://www.youtube.com/youtubei/v1/player?key={api_key}");
    println!("   Request URL: {url}");

    let context = RequestContext {
        client_name: Some("WEB".to_string()),
        is_player_request: true,
        ..Default::default()
    };

    let request = http_client
        .client()
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .build()?;

    println!("   Request headers: {:?}", request.headers());

    match http_client.execute_with_context(request, context).await {
        Ok(response) => {
            println!("✅ HTTP request successful!");
            println!("   Status: {}", response.status());
            println!("   Headers: {:?}", response.headers());

            match response.text().await {
                Ok(body) => {
                    println!("   Response body length: {} bytes", body.len());

                    // Try to parse as JSON
                    match serde_json::from_str::<serde_json::Value>(&body) {
                        Ok(json) => {
                            println!("   ✅ Valid JSON response");

                            // Check for playability status
                            if let Some(status) = json.get("playabilityStatus") {
                                println!(
                                    "   Playability status: {}",
                                    serde_json::to_string_pretty(status)?
                                );
                            }

                            // Check for video details
                            if let Some(details) = json.get("videoDetails") {
                                println!(
                                    "   Video details found: {}",
                                    details.get("title").unwrap_or(&serde_json::Value::Null)
                                );
                            }

                            // Check for streaming data
                            if let Some(streaming_data) = json.get("streamingData") {
                                println!("   ✅ Streaming data found");
                                if let Some(formats) = streaming_data.get("formats") {
                                    println!(
                                        "   Regular formats count: {}",
                                        formats.as_array().map(|a| a.len()).unwrap_or(0)
                                    );
                                }
                                if let Some(adaptive_formats) =
                                    streaming_data.get("adaptiveFormats")
                                {
                                    println!(
                                        "   Adaptive formats count: {}",
                                        adaptive_formats.as_array().map(|a| a.len()).unwrap_or(0)
                                    );
                                }
                            } else {
                                println!("   ❌ No streaming data found");
                            }
                        }
                        Err(e) => {
                            println!("   ❌ Invalid JSON response: {e}");
                            println!("   First 500 chars: {}", &body[..body.len().min(500)]);
                        }
                    }
                }
                Err(e) => {
                    println!("   ❌ Failed to read response body: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ HTTP request failed: {e}");
        }
    }

    // Test 3: Test with different video IDs
    println!("\n🎵 Test 3: Testing different video types");
    let test_videos = vec![
        ("dQw4w9WgXcQ", "Rick Roll - Regular video"),
        ("jNQXAC9IVRw", "Me at the zoo - First YouTube video"),
        ("invalid_id", "Invalid video ID"),
    ];

    for (video_id, description) in test_videos {
        println!("\n   Testing: {description} ({video_id})");
        match web_client.load_track_info_from_innertube(video_id).await {
            Ok(track_info) => {
                println!("   ✅ Success: {}", track_info.title);
            }
            Err(e) => {
                println!("   ❌ Failed: {e}");
            }
        }
    }

    println!("\n🎉 Debug session complete!");

    Ok(())
}
