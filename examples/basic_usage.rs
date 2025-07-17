use youtube_source_rs::{YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Create a YouTube source manager with default options
    let options = YoutubeSourceOptions::default()
        .set_allow_search(true)
        .set_allow_direct_video_ids(true);

    let manager = YoutubeAudioSourceManager::with_options(options);

    println!("YouTube Source Manager created successfully!");
    println!("Available clients: {}", manager.clients.len());

    // TODO: Once implementation is complete, you can test loading items:
    // let result = manager.load_item("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
    // println!("Load result: {:?}", result);

    Ok(())
}
