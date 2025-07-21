use youtube_source_rs::client::{NonMusicClient, WebClient};
use youtube_source_rs::http::RequestContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    println!("🔍 Debug Stream Format Parsing");
    println!("===============================");

    let web_client = WebClient::new().expect("Failed to create WebClient");
    let video_id = "dQw4w9WgXcQ"; // Rick Roll

    println!("🎬 Analyzing video: {video_id}");

    // Make direct API call to examine streaming data
    let http_client = web_client.get_http_client();
    let client_config = web_client.get_client_config();

    let payload = serde_json::json!({
        "context": client_config.to_context_json(),
        "videoId": video_id
    });

    let api_key = client_config.get_api_key().unwrap_or("NO_API_KEY");
    let url = format!("https://www.youtube.com/youtubei/v1/player?key={api_key}");

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

    match http_client.execute_with_context(request, context).await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    match serde_json::from_str::<serde_json::Value>(&body) {
                        Ok(json) => {
                            println!("✅ Got API response");

                            // Check streaming data structure
                            if let Some(streaming_data) = json.get("streamingData") {
                                println!("✅ Found streamingData");

                                // Check adaptive formats
                                if let Some(adaptive_formats) =
                                    streaming_data.get("adaptiveFormats")
                                {
                                    if let Some(adaptive_array) = adaptive_formats.as_array() {
                                        println!("📊 Adaptive formats: {}", adaptive_array.len());

                                        // First, let's find audio formats
                                        let audio_formats: Vec<_> = adaptive_array
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, format)| {
                                                format
                                                    .get("mimeType")
                                                    .and_then(|m| m.as_str())
                                                    .map(|s| s.starts_with("audio/"))
                                                    .unwrap_or(false)
                                            })
                                            .collect();

                                        println!(
                                            "   🎵 Found {} audio formats out of {} total",
                                            audio_formats.len(),
                                            adaptive_array.len()
                                        );

                                        for (i, (_, format)) in
                                            audio_formats.iter().enumerate().take(5)
                                        {
                                            println!("\n   Audio Format {}:", i + 1);

                                            if let Some(itag) = format.get("itag") {
                                                println!("     🏷️  itag: {itag}");
                                            }

                                            if let Some(mime_type) = format.get("mimeType") {
                                                println!("     📄 mimeType: {mime_type}");
                                            }

                                            if let Some(bitrate) = format.get("bitrate") {
                                                println!("     🎵 bitrate: {bitrate}");
                                            }

                                            if let Some(url) = format.get("url") {
                                                let url_str = url.as_str().unwrap_or("invalid");
                                                println!(
                                                    "     🔗 url: {}...",
                                                    &url_str[..url_str.len().min(80)]
                                                );
                                            } else {
                                                println!("     ❌ No direct URL found");

                                                // Check for encrypted signature
                                                if let Some(signature) = format.get("s") {
                                                    let sig_str =
                                                        signature.as_str().unwrap_or("invalid");
                                                    let truncated =
                                                        &sig_str[..sig_str.len().min(20)];
                                                    println!("     🔐 signature: {truncated}...");
                                                }

                                                if let Some(signature_key) = format.get("sp") {
                                                    println!(
                                                        "     🔑 signature key: {signature_key}"
                                                    );
                                                }

                                                // Check for base URL
                                                if let Some(base_url) = format.get("url") {
                                                    let url_str =
                                                        base_url.as_str().unwrap_or("invalid");
                                                    println!(
                                                        "     🔗 base URL: {}...",
                                                        &url_str[..url_str.len().min(80)]
                                                    );
                                                }

                                                // Check for n parameter
                                                if let Some(n_param) = format.get("n") {
                                                    println!("     🔢 n parameter: {n_param}");
                                                }
                                            }

                                            // Check if it's audio
                                            if let Some(mime_type) = format.get("mimeType") {
                                                let mime_str = mime_type.as_str().unwrap_or("");
                                                if mime_str.starts_with("audio/") {
                                                    println!("     🎵 This is an AUDIO format");
                                                } else {
                                                    println!("     🎬 This is a VIDEO format");
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    println!("❌ No adaptiveFormats found");
                                }

                                // Check regular formats
                                if let Some(formats) = streaming_data.get("formats") {
                                    if let Some(formats_array) = formats.as_array() {
                                        println!("\n📊 Regular formats: {}", formats_array.len());

                                        for (i, format) in formats_array.iter().enumerate().take(3)
                                        {
                                            println!("\n   Regular Format {}:", i + 1);

                                            if let Some(itag) = format.get("itag") {
                                                println!("     🏷️  itag: {itag}");
                                            }

                                            if let Some(mime_type) = format.get("mimeType") {
                                                println!("     📄 mimeType: {mime_type}");
                                            }

                                            if let Some(url) = format.get("url") {
                                                let url_str = url.as_str().unwrap_or("invalid");
                                                println!(
                                                    "     🔗 url: {}...",
                                                    &url_str[..url_str.len().min(80)]
                                                );
                                            } else {
                                                println!("     ❌ No direct URL found");
                                            }
                                        }
                                    }
                                } else {
                                    println!("❌ No regular formats found");
                                }
                            } else {
                                println!("❌ No streamingData found in response");

                                // Check what keys are available
                                println!("Available top-level keys:");
                                if let Some(obj) = json.as_object() {
                                    for key in obj.keys().take(10) {
                                        println!("  - {key}");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ Failed to parse JSON: {e}");
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to read response: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ API request failed: {e}");
        }
    }

    println!("\n🎉 Debug complete!");

    Ok(())
}
