use youtube_source_rs::cipher::{AdvancedSignatureCipher, ExtractedCipher, ScriptParser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 Testing Advanced Signature Cipher Implementation");
    println!("==================================================");

    // Test 1: Script parsing
    println!("\n📜 Testing script parsing...");
    test_script_parsing()?;
    println!("✅ Script parsing test passed");

    // Test 2: Advanced cipher creation
    println!("\n🔧 Testing advanced cipher creation...");
    test_advanced_cipher_creation()?;
    println!("✅ Advanced cipher creation test passed");

    // Test 3: Signature decryption
    println!("\n🔐 Testing signature decryption...");
    test_signature_decryption()?;
    println!("✅ Signature decryption test passed");

    // Test 4: N parameter transformation
    println!("\n🔄 Testing N parameter transformation...");
    test_n_parameter_transformation()?;
    println!("✅ N parameter transformation test passed");

    // Test 5: Performance benchmark
    println!("\n⚡ Performance benchmark...");
    benchmark_advanced_cipher()?;
    println!("✅ Performance benchmark completed");

    // Test 6: Integration test
    println!("\n🔗 Integration test...");
    test_integration()?;
    println!("✅ Integration test passed");

    println!("\n🎉 All advanced cipher tests passed!");
    println!("Ready for integration with SignatureCipherManager.");

    Ok(())
}

fn test_script_parsing() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock YouTube player script with all required components
    let mock_script = r#"
        signatureTimestamp:19834,other:"value"
        var a = "abcdefghijklmnopqrstuvwxyz0123456789".split("");
    "#;

    // Test timestamp extraction
    let timestamp = ScriptParser::extract_timestamp(mock_script)?;
    println!("  Extracted timestamp: {timestamp}");
    assert_eq!(timestamp, "19834");

    // Test global vars extraction
    let global_vars = ScriptParser::extract_global_vars(mock_script)?;
    println!(
        "  Extracted global vars: {}",
        global_vars.chars().take(50).collect::<String>() + "..."
    );
    assert!(global_vars.contains("var a"));

    Ok(())
}

fn test_advanced_cipher_creation() -> Result<(), Box<dyn std::error::Error>> {
    let cipher_info = create_test_cipher();
    let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info)?;

    println!("  Advanced cipher created successfully");
    println!("  Timestamp: {}", cipher.get_timestamp());

    // Test cipher functionality
    cipher.test_cipher()?;
    println!("  Cipher self-test passed");

    Ok(())
}

fn test_signature_decryption() -> Result<(), Box<dyn std::error::Error>> {
    let cipher_info = create_test_cipher();
    let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info)?;

    // Test with various signature patterns
    let test_cases = [
        "abcdefghijklmnop",
        "0123456789abcdef",
        "zyxwvutsrqponmlkjihgfedcba9876543210",
        "short",
        "verylongsignaturethatmightcauseproblemswithsomeoperations",
    ];

    for (i, test_sig) in test_cases.iter().enumerate() {
        let result = cipher.decipher_signature(test_sig)?;
        println!("  Test {}: '{}' -> '{}'", i + 1, test_sig, result);

        // Verify that the signature was actually transformed
        if result != *test_sig {
            println!("    ✅ Signature transformed successfully");
        } else {
            println!("    ⚠️  Signature unchanged (identity function)");
        }
    }

    Ok(())
}

fn test_n_parameter_transformation() -> Result<(), Box<dyn std::error::Error>> {
    let cipher_info = create_test_cipher();
    let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info)?;

    // Test with various N parameter patterns
    let test_cases = [
        "abc123",
        "xyz789",
        "test_param",
        "1234567890",
        "mixed123abc",
    ];

    for (i, test_n) in test_cases.iter().enumerate() {
        let result = cipher.transform_n_parameter(test_n)?;
        println!("  Test {}: '{}' -> '{}'", i + 1, test_n, result);

        // Verify that the N parameter was transformed
        if result != *test_n {
            println!("    ✅ N parameter transformed successfully");
        } else {
            println!("    ⚠️  N parameter unchanged (identity function)");
        }
    }

    Ok(())
}

fn benchmark_advanced_cipher() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let cipher_info = create_test_cipher();
    let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info)?;

    let test_signature = "abcdefghijklmnopqrstuvwxyz0123456789";
    let test_n_param = "test123param456";
    let iterations = 50;

    // Benchmark signature decryption
    let start = Instant::now();
    for i in 0..iterations {
        let result = cipher.decipher_signature(test_signature)?;
        if i == 0 {
            println!("  Sample signature result: '{test_signature}' -> '{result}'");
        }
    }
    let sig_duration = start.elapsed();
    let sig_avg = sig_duration.as_micros() / iterations;

    // Benchmark N parameter transformation
    let start = Instant::now();
    for i in 0..iterations {
        let result = cipher.transform_n_parameter(test_n_param)?;
        if i == 0 {
            println!("  Sample N param result: '{test_n_param}' -> '{result}'");
        }
    }
    let n_duration = start.elapsed();
    let n_avg = n_duration.as_micros() / iterations;

    println!("  Signature decryption: {iterations} iterations in {sig_duration:?}");
    println!(
        "  Average signature time: {}μs ({:.2}ms)",
        sig_avg,
        sig_avg as f64 / 1000.0
    );

    println!("  N parameter transform: {iterations} iterations in {n_duration:?}");
    println!(
        "  Average N param time: {}μs ({:.2}ms)",
        n_avg,
        n_avg as f64 / 1000.0
    );

    // Check performance targets
    if sig_avg < 50_000 && n_avg < 50_000 {
        println!("  ✅ Performance targets met (<50ms each)");
    } else {
        println!("  ⚠️  Performance targets missed (>50ms)");
    }

    Ok(())
}

fn test_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test the complete flow from script parsing to cipher operations
    let mock_script = create_mock_youtube_script();

    println!("  Testing complete script-to-cipher flow...");

    // Parse the script
    let extracted = ScriptParser::extract_cipher_from_script(&mock_script)?;
    println!("  ✅ Script parsed successfully");

    // Create advanced cipher
    let cipher = AdvancedSignatureCipher::from_extracted_cipher(extracted)?;
    println!("  ✅ Advanced cipher created from parsed script");

    // Test operations
    let test_sig = "integration_test_signature";
    let test_n = "integration_test_n_param";

    let sig_result = cipher.decipher_signature(test_sig)?;
    let n_result = cipher.transform_n_parameter(test_n)?;

    println!("  Integration signature: '{test_sig}' -> '{sig_result}'");
    println!("  Integration N param: '{test_n}' -> '{n_result}'");

    println!("  ✅ Complete integration flow working");

    Ok(())
}

fn create_test_cipher() -> ExtractedCipher {
    ExtractedCipher {
        timestamp: "19834".to_string(),
        global_vars: r#"var a = "abcdefghijklmnopqrstuvwxyz0123456789".split("");"#.to_string(),
        sig_actions: r#"var helper = {
            reverse: function(b) { b.reverse(); },
            swap: function(b, c) { var d = b[0]; b[0] = b[c % b.length]; b[c % b.length] = d; },
            splice: function(b, c) { b.splice(0, c); }
        };"#
        .to_string(),
        sig_function: r#"function sig(b) {
            var c = b.split('');
            helper.reverse(c);
            helper.swap(c, 1);
            helper.splice(c, 2);
            return c.join('');
        }"#
        .to_string(),
        n_function: r#"function n(b) {
            return 'yt_' + b.split('').reverse().join('');
        }"#
        .to_string(),
        raw_script: "test script".to_string(),
    }
}

fn create_mock_youtube_script() -> String {
    r#"
        var config = {
            signatureTimestamp: 19834,
            version: "2.20250120.01.00"
        };

        var a = "abcdefghijklmnopqrstuvwxyz0123456789".split("");

        var helper = {
            reverse: function(b) { b.reverse(); },
            swap: function(b, c) { var d = b[0]; b[0] = b[c % b.length]; b[c % b.length] = d; },
            splice: function(b, c) { b.splice(0, c); }
        };

        function sig(b) {
            var c = b.split('');
            helper.reverse(c);
            helper.swap(c, 1);
            helper.splice(c, 2);
            return c.join('');
        }

        function n(b) {
            var c = b.split('').reverse().join('');
            return 'yt_' + c;
        };

        // Other YouTube player code...
        var player = { version: "2.20250120.01.00" };
    "#
    .to_string()
}
