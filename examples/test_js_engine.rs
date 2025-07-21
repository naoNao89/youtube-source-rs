use youtube_source_rs::cipher::JavaScriptEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 Testing JavaScript Engine Integration");
    println!("========================================");

    // Create JavaScript engine
    let engine = JavaScriptEngine::new()?;
    println!("✅ JavaScript engine created successfully");

    // Test basic functionality
    println!("\n🔧 Testing basic functionality...");
    engine.test_engine()?;
    println!("✅ Basic functionality test passed");

    // Test cipher-like operations
    println!("\n🔐 Testing cipher operations...");
    test_cipher_operations(&engine)?;
    println!("✅ Cipher operations test passed");

    // Test N parameter transformation
    println!("\n🔄 Testing N parameter transformation...");
    test_n_parameter_transformation(&engine)?;
    println!("✅ N parameter transformation test passed");

    // Performance benchmark
    println!("\n⚡ Performance benchmark...");
    benchmark_performance(&engine)?;
    println!("✅ Performance benchmark completed");

    println!("\n🎉 All JavaScript engine tests passed!");
    println!("Ready for advanced signature cipher implementation.");

    Ok(())
}

fn test_cipher_operations(engine: &JavaScriptEngine) -> Result<(), Box<dyn std::error::Error>> {
    // Test 1: Simple reverse operation
    let script1 = r#"
        function reverse(str) {
            return str.split('').reverse().join('');
        }
    "#;

    let result1 = engine.execute_cipher_function(script1, "reverse", "abcdef")?;
    println!("  Reverse test: 'abcdef' -> '{result1}'");
    assert_eq!(result1, "fedcba");

    // Test 2: Complex cipher operation (similar to YouTube patterns)
    let script2 = r#"
        function complexCipher(sig) {
            var a = sig.split('');
            
            // Operation 1: Reverse
            a.reverse();
            
            // Operation 2: Swap positions
            var temp = a[0];
            a[0] = a[1];
            a[1] = temp;
            
            // Operation 3: Slice from position 2
            a.splice(0, 2);
            
            return a.join('');
        }
    "#;

    let result2 = engine.execute_cipher_function(script2, "complexCipher", "0123456789")?;
    println!("  Complex cipher: '0123456789' -> '{result2}'");

    // Test 3: YouTube-style cipher with helper functions
    let script3 = r#"
        var helper = {
            reverse: function(a) {
                a.reverse();
            },
            swap: function(a, b) {
                var c = a[0];
                a[0] = a[b % a.length];
                a[b % a.length] = c;
            },
            splice: function(a, b) {
                a.splice(0, b);
            }
        };
        
        function youtubeCipher(sig) {
            var a = sig.split('');
            helper.reverse(a);
            helper.swap(a, 1);
            helper.splice(a, 2);
            return a.join('');
        }
    "#;

    let result3 = engine.execute_cipher_function(script3, "youtubeCipher", "abcdefghij")?;
    println!("  YouTube-style cipher: 'abcdefghij' -> '{result3}'");

    Ok(())
}

fn test_n_parameter_transformation(
    engine: &JavaScriptEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    // Test 1: Simple N transformation
    let script1 = r#"
        function transformN(n) {
            return 'yt_' + n.split('').reverse().join('');
        }
    "#;

    let result1 = engine.execute_n_transform_function(script1, "transformN", "abc123")?;
    println!("  Simple N transform: 'abc123' -> '{result1}'");
    assert_eq!(result1, "yt_321cba");

    // Test 2: Complex N transformation (YouTube-like)
    let script2 = r#"
        function complexNTransform(n) {
            var chars = n.split('');
            var result = [];
            
            for (var i = 0; i < chars.length; i++) {
                var char = chars[i];
                if (char >= '0' && char <= '9') {
                    result.push(String.fromCharCode(char.charCodeAt(0) + 1));
                } else {
                    result.push(char);
                }
            }
            
            return result.reverse().join('');
        }
    "#;

    let result2 = engine.execute_n_transform_function(script2, "complexNTransform", "abc123")?;
    println!("  Complex N transform: 'abc123' -> '{result2}'");

    Ok(())
}

fn benchmark_performance(engine: &JavaScriptEngine) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let script = r#"
        function benchmarkCipher(sig) {
            var a = sig.split('');
            a.reverse();
            var temp = a[0];
            a[0] = a[1];
            a[1] = temp;
            a.splice(0, 2);
            return a.join('');
        }
    "#;

    let test_signature = "abcdefghijklmnopqrstuvwxyz0123456789";
    let iterations = 100;

    let start = Instant::now();

    for i in 0..iterations {
        let result = engine.execute_cipher_function(script, "benchmarkCipher", test_signature)?;
        if i == 0 {
            println!("  Sample result: '{test_signature}' -> '{result}'");
        }
    }

    let duration = start.elapsed();
    let avg_time = duration.as_micros() / iterations;

    println!("  {iterations} iterations completed in {duration:?}");
    println!(
        "  Average execution time: {}μs ({:.2}ms)",
        avg_time,
        avg_time as f64 / 1000.0
    );

    if avg_time < 50_000 {
        // 50ms target
        println!("  ✅ Performance target met (<50ms)");
    } else {
        println!("  ⚠️  Performance target missed (>50ms)");
    }

    Ok(())
}
