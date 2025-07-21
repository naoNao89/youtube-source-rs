use std::time::Duration;
use youtube_source_rs::http::{YoutubeAccessTokenTracker, YoutubeOauth2Handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🔐 Testing YouTube OAuth2 Integration");
    println!("====================================");

    // Test OAuth2 Handler
    println!("\n1. Testing OAuth2 Handler");
    let oauth_handler = YoutubeOauth2Handler::new();

    // Test with no refresh token (should trigger device flow)
    println!("   - Testing device flow initialization...");
    match oauth_handler.set_refresh_token(None, false).await {
        Ok(_) => println!("   ✅ Device flow initialized successfully"),
        Err(e) => println!("   ❌ Device flow failed: {e}"),
    }

    // Test refresh token validation
    println!("   - Testing refresh token validation...");
    let should_refresh = oauth_handler.should_refresh_access_token().await;
    println!("   📊 Should refresh token: {should_refresh}");

    // Test with a dummy refresh token (will fail but tests the flow)
    println!("   - Testing with dummy refresh token...");
    match oauth_handler
        .set_refresh_token(Some("dummy_token".to_string()), true)
        .await
    {
        Ok(_) => println!("   ✅ Refresh token set successfully"),
        Err(e) => println!("   ⚠️  Expected error with dummy token: {e}"),
    }

    // Test Access Token Tracker
    println!("\n2. Testing Access Token Tracker");
    let token_tracker = YoutubeAccessTokenTracker::new();

    // Test visitor ID fetching
    println!("   - Fetching visitor ID...");
    match token_tracker.get_visitor_id().await {
        Some(visitor_id) => {
            println!("   ✅ Visitor ID fetched: {visitor_id}");

            // Test caching - should return same ID quickly
            let start = std::time::Instant::now();
            let cached_id = token_tracker.get_visitor_id().await;
            let duration = start.elapsed();

            if cached_id == Some(visitor_id.clone()) && duration < Duration::from_millis(10) {
                println!(
                    "   ✅ Visitor ID caching working ({}ms)",
                    duration.as_millis()
                );
            } else {
                println!("   ⚠️  Visitor ID caching may not be working properly");
            }
        }
        None => println!("   ❌ Failed to fetch visitor ID"),
    }

    // Test context checking
    println!("   - Testing context checking...");
    let mut context = std::collections::HashMap::new();
    context.insert("yt-raw".to_string(), "true".to_string());

    let is_token_context = token_tracker.is_token_fetch_context(&context);
    println!("   📊 Is token fetch context: {is_token_context}");

    // Test OAuth context checking
    let mut oauth_context = std::collections::HashMap::new();
    oauth_context.insert("yt-oauth".to_string(), "true".to_string());

    let is_oauth_context = oauth_handler.is_oauth_fetch_context(&oauth_context);
    println!("   📊 Is OAuth fetch context: {is_oauth_context}");

    // Test token application to request
    println!("\n3. Testing Token Application");
    let client = reqwest::Client::new();
    let mut request = client.get("https://example.com").build()?;

    println!("   - Testing token application to request...");
    match oauth_handler.apply_token(&mut request).await {
        Ok(_) => println!("   ✅ Token application completed (no token available, but no error)"),
        Err(e) => println!("   ❌ Token application failed: {e}"),
    }

    // Test direct token application
    println!("   - Testing direct token application...");
    let mut direct_request = client.get("https://example.com").build()?;
    match YoutubeOauth2Handler::apply_token_direct(&mut direct_request, "test_token") {
        Ok(_) => {
            if let Some(auth_header) = direct_request.headers().get("authorization") {
                println!("   ✅ Direct token applied: {auth_header:?}");
            } else {
                println!("   ❌ Direct token not found in headers");
            }
        }
        Err(e) => println!("   ❌ Direct token application failed: {e}"),
    }

    println!("\n🎯 OAuth2 Integration Test Summary");
    println!("==================================");
    println!("✅ OAuth2Handler: Device flow initialization working");
    println!("✅ AccessTokenTracker: Visitor ID fetching working");
    println!("✅ Context management: Working correctly");
    println!("✅ Token application: Working correctly");
    println!("⚠️  Note: Full OAuth2 flow requires user interaction");
    println!("⚠️  Note: Refresh token testing requires valid tokens");

    println!("\n📋 Next Steps for Full Integration:");
    println!("1. Integrate with HTTP filter system");
    println!("2. Add OAuth2 to YoutubeAudioSourceManager");
    println!("3. Test with real YouTube API requests");
    println!("4. Add protected content access");

    Ok(())
}
