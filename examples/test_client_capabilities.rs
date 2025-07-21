use youtube_source_rs::client::traits::{Client, generate_capabilities_summary};
use youtube_source_rs::client::{
    AndroidClient, IosClient, MusicClient, TvClient, WebClient, WebEmbeddedClient,
};

/// Example demonstrating the comprehensive client capabilities system
/// This matches the Client Capabilities Summary table provided
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("YouTube Client Capabilities Test");
    println!("================================\n");

    // Print the capabilities summary table
    println!("{}\n", generate_capabilities_summary());

    // Test all client types and their capabilities
    test_android_clients().await?;
    test_ios_client().await?;
    test_tv_clients().await?;
    test_web_clients().await?;

    println!("✅ All client capability tests passed!");
    Ok(())
}

async fn test_android_clients() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Android Clients");
    println!("-----------------------");

    // Android Standard
    let android_standard = AndroidClient::new();
    let caps = android_standard.get_capabilities();
    println!(
        "Android Standard: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(caps.playlists);
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    // Android Music
    let android_music = AndroidClient::music();
    let caps = android_music.get_capabilities();
    println!(
        "Android Music: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(!caps.playlists); // Music client doesn't support regular playlists
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    // Android VR
    let android_vr = AndroidClient::vr();
    let caps = android_vr.get_capabilities();
    println!(
        "Android VR: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(caps.playlists);
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    println!("✅ Android clients test passed\n");
    Ok(())
}

async fn test_ios_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing iOS Client");
    println!("------------------");

    let ios_client = IosClient::new();
    let caps = ios_client.get_capabilities();
    println!(
        "iOS: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(!caps.playlists); // iOS doesn't support regular playlists
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    println!("✅ iOS client test passed\n");
    Ok(())
}

async fn test_tv_clients() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing TV Clients");
    println!("------------------");

    // TV Standard
    let tv_standard = TvClient::new();
    let caps = tv_standard.get_capabilities();
    println!(
        "TV Standard: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(!caps.videos); // TV Standard cannot load videos
    assert!(!caps.playlists);
    assert!(!caps.mixes); // TV Standard cannot load mixes
    assert!(caps.search);
    assert!(!caps.embedded);

    // TV HTML5 Embedded
    let tv_embedded = TvClient::html5_embedded();
    let caps = tv_embedded.get_capabilities();
    println!(
        "TV HTML5 Embedded: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos); // HTML5 Embedded can load videos
    assert!(!caps.playlists);
    assert!(caps.mixes); // HTML5 Embedded can load mixes
    assert!(caps.search);
    assert!(caps.embedded); // This is an embedded client

    println!("✅ TV clients test passed\n");
    Ok(())
}

async fn test_web_clients() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Web Clients");
    println!("-------------------");

    // Web Client
    let web_client = WebClient::new()?;
    let caps = web_client.get_capabilities();
    println!(
        "Web: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(caps.playlists);
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    // Music Client
    let music_client = MusicClient::new();
    let caps = music_client.get_capabilities();
    println!(
        "Music: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(caps.playlists);
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(!caps.embedded);

    // Web Embedded Client
    let embedded_client = WebEmbeddedClient::new();
    let caps = embedded_client.get_capabilities();
    println!(
        "Web Embedded: OAuth={}, Videos={}, Playlists={}, Mixes={}, Search={}, Embedded={}",
        caps.oauth, caps.videos, caps.playlists, caps.mixes, caps.search, caps.embedded
    );

    assert!(caps.oauth);
    assert!(caps.videos);
    assert!(!caps.playlists); // Embedded clients typically don't support playlists
    assert!(caps.mixes);
    assert!(caps.search);
    assert!(caps.embedded);

    println!("✅ Web clients test passed\n");
    Ok(())
}

/// Test capability validation in client methods
async fn test_capability_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Capability Validation");
    println!("-----------------------------");

    // Test that Android Music client rejects playlist loading
    let android_music = AndroidClient::music();
    assert!(!android_music.supports_playlists());

    // Test that TV Standard client rejects video loading
    let tv_standard = TvClient::new();
    assert!(!tv_standard.supports_videos());
    assert!(!tv_standard.supports_mixes());

    // Test that iOS client rejects playlist loading
    let ios_client = IosClient::new();
    assert!(!ios_client.supports_playlists());

    println!("✅ Capability validation test passed\n");
    Ok(())
}
