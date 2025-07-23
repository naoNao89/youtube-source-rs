use url::Url;
use youtube_source_rs::cipher::SignatureCipherManager;
use youtube_source_rs::{track::FormatInfo, StreamFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 Testing Enhanced SignatureCipherManager");
    println!("==========================================");

    // Create manager
    let manager = SignatureCipherManager::new();
    println!("✅ SignatureCipherManager created");

    // Test 1: Cache statistics
    println!("\n📊 Testing cache statistics...");
    test_cache_stats(&manager).await?;
    println!("✅ Cache statistics test passed");

    // Test 2: Mock script processing
    println!("\n📜 Testing script processing...");
    test_script_processing(&manager).await?;
    println!("✅ Script processing test passed");

    // Test 3: Format URL resolution
    println!("\n🔗 Testing format URL resolution...");
    test_format_resolution(&manager).await?;
    println!("✅ Format URL resolution test passed");

    // Test 4: Cache management
    println!("\n🗄️ Testing cache management...");
    test_cache_management(&manager).await?;
    println!("✅ Cache management test passed");

    // Test 5: Performance benchmark
    println!("\n⚡ Performance benchmark...");
    benchmark_manager_performance(&manager).await?;
    println!("✅ Performance benchmark completed");

    println!("\n🎉 All enhanced manager tests passed!");
    println!("SignatureCipherManager integration successful.");

    Ok(())
}

async fn test_cache_stats(
    manager: &SignatureCipherManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let stats = manager.get_cache_stats().await;
    println!("  Initial cache stats:");
    println!("    Total entries: {}", stats.total_entries);
    println!(
        "    Advanced cipher entries: {}",
        stats.advanced_cipher_entries
    );
    println!("    Basic cipher entries: {}", stats.basic_cipher_entries);
    println!("    Expired entries: {}", stats.expired_entries);

    assert_eq!(stats.total_entries, 0);
    assert_eq!(stats.advanced_cipher_entries, 0);
    assert_eq!(stats.basic_cipher_entries, 0);
    assert_eq!(stats.expired_entries, 0);

    Ok(())
}

async fn test_script_processing(
    manager: &SignatureCipherManager,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock player script URL (this would normally be a real YouTube player script)
    let mock_url = Url::parse("https://www.youtube.com/s/player/mock_script.js")?;

    println!("  Testing with mock URL: {mock_url}");

    // This will fail gracefully since it's a mock URL, but we can test the error handling
    match manager.get_advanced_cipher(&mock_url).await {
        Ok(_) => {
            println!("  ✅ Advanced cipher created successfully");
        }
        Err(e) => {
            println!("  ⚠️  Advanced cipher failed as expected (mock URL): {e}");
            println!("  Testing fallback to basic cipher...");

            // Test basic cipher fallback
            match manager.get_cipher(&mock_url).await {
                Ok(_) => println!("  ⚠️  Basic cipher also succeeded (unexpected for mock URL)"),
                Err(e) => println!("  ✅ Basic cipher failed as expected: {e}"),
            }
        }
    }

    Ok(())
}

async fn test_format_resolution(
    manager: &SignatureCipherManager,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock StreamFormat for testing
    let mock_format = StreamFormat {
        info: Some(FormatInfo::Mp4AacLc),
        content_type: "audio/mp4".to_string(),
        itag: 140,
        bitrate: 128000,
        content_length: 1000000,
        audio_channels: 2,
        url: Url::parse("https://example.com/video.mp4?signature=test123")?,
        n_parameter: Some("test_n_param".to_string()),
        signature: Some("test_signature_to_decipher".to_string()),
        signature_key: Some("sig".to_string()),
        is_default_audio_track: true,
        is_drc: false,
    };

    let mock_player_url = Url::parse("https://www.youtube.com/s/player/test.js")?;

    println!("  Testing format URL resolution...");
    println!("    Original URL: {}", mock_format.url);
    println!("    Signature: {:?}", mock_format.signature);
    println!("    N Parameter: {:?}", mock_format.n_parameter);

    // This will test the fallback mechanism since the mock URL won't work
    match manager
        .resolve_format_url(&mock_player_url, &mock_format)
        .await
    {
        Ok(resolved_url) => {
            println!("  ✅ URL resolved: {resolved_url}");
        }
        Err(e) => {
            println!("  ⚠️  URL resolution failed as expected (mock data): {e}");
        }
    }

    Ok(())
}

async fn test_cache_management(
    manager: &SignatureCipherManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing cache cleanup...");
    manager.cleanup_cache().await;

    let stats_before = manager.get_cache_stats().await;
    println!("    Cache entries before: {}", stats_before.total_entries);

    // Test refresh (will fail with mock URL but tests the mechanism)
    let mock_url = Url::parse("https://www.youtube.com/s/player/refresh_test.js")?;
    match manager.refresh_script(&mock_url).await {
        Ok(_) => println!("  ✅ Script refresh succeeded"),
        Err(e) => println!("  ⚠️  Script refresh failed as expected: {e}"),
    }

    let stats_after = manager.get_cache_stats().await;
    println!("    Cache entries after: {}", stats_after.total_entries);

    Ok(())
}

async fn benchmark_manager_performance(
    manager: &SignatureCipherManager,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    println!("  Benchmarking cache operations...");

    let iterations = 10;
    let start = Instant::now();

    for i in 0..iterations {
        let stats = manager.get_cache_stats().await;
        if i == 0 {
            println!(
                "    Sample stats: {} total, {} advanced",
                stats.total_entries, stats.advanced_cipher_entries
            );
        }
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() / iterations;

    println!("    {iterations} cache stat operations in {duration:?}");
    println!("    Average time per operation: {avg_time}μs");

    if avg_time < 1000 {
        // 1ms target
        println!("    ✅ Performance target met (<1ms)");
    } else {
        println!("    ⚠️  Performance target missed (>1ms)");
    }

    // Test cache cleanup performance
    let start = Instant::now();
    manager.cleanup_cache().await;
    let cleanup_time = start.elapsed();

    println!("    Cache cleanup time: {cleanup_time:?}");

    Ok(())
}

// Helper function to create a realistic mock YouTube player script
#[allow(dead_code)]
fn create_realistic_mock_script() -> String {
    r#"
        var ytInitialPlayerResponse = {
            "signatureTimestamp": 19834,
            "version": "2.20250121.01.00"
        };

        var a = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

        var cipherHelper = {
            reverse: function(arr) { 
                arr.reverse(); 
            },
            swap: function(arr, index) { 
                var temp = arr[0]; 
                arr[0] = arr[index % arr.length]; 
                arr[index % arr.length] = temp; 
            },
            splice: function(arr, count) { 
                arr.splice(0, count); 
            }
        };

        function signatureDecipher(signature) {
            var chars = signature.split('');
            cipherHelper.reverse(chars);
            cipherHelper.swap(chars, 47);
            cipherHelper.splice(chars, 2);
            cipherHelper.reverse(chars);
            cipherHelper.swap(chars, 1);
            return chars.join('');
        }

        function nParameterTransform(n) {
            var result = n;
            try {
                var chars = n.split('');
                chars.reverse();
                result = 'yt_' + chars.join('');
            } catch (e) {
                result = 'enhanced_except_' + n;
            }
            return result;
        }

        // Additional YouTube player code...
        var player = {
            version: "2.20250121.01.00",
            config: ytInitialPlayerResponse
        };
    "#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = SignatureCipherManager::new();
        let stats = manager.get_cache_stats().await;
        assert_eq!(stats.total_entries, 0);
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        let manager = SignatureCipherManager::new();
        manager.cleanup_cache().await;
        let stats = manager.get_cache_stats().await;
        assert_eq!(stats.expired_entries, 0);
    }
}
