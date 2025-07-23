use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::runtime::Runtime;
use url::Url;
use youtube_source_rs::cipher::SignatureCipherManager;

fn bench_cipher_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("cipher_creation");

    group.bench_function("signature_cipher_manager", |b| {
        b.iter(|| {
            let manager = SignatureCipherManager::new();
            black_box(manager)
        })
    });

    group.finish();
}

fn bench_basic_cipher_operations(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("basic_cipher");

    // Sample cipher operations (these would be real cipher operations in practice)
    let test_signatures = vec![
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "1234567890abcdefghijklmnop",
        "zyxwvutsrqponmlkjihgfedcba",
    ];

    for (i, signature) in test_signatures.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("reverse_operation", i),
            signature,
            |b, sig| {
                b.iter(|| {
                    // Simulate reverse operation
                    let reversed: String = sig.chars().rev().collect();
                    black_box(reversed)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("swap_operation", i),
            signature,
            |b, sig| {
                b.iter(|| {
                    // Simulate swap operation
                    let mut chars: Vec<char> = sig.chars().collect();
                    if chars.len() > 1 {
                        let len = chars.len();
                        chars.swap(0, len - 1);
                    }
                    let swapped: String = chars.into_iter().collect();
                    black_box(swapped)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("slice_operation", i),
            signature,
            |b, sig| {
                b.iter(|| {
                    // Simulate slice operation
                    let sliced = if sig.len() > 2 {
                        &sig[1..sig.len() - 1]
                    } else {
                        sig
                    };
                    black_box(sliced.to_string())
                })
            },
        );
    }

    group.finish();
}

fn bench_url_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_operations");

    let test_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s",
        "https://youtu.be/dQw4w9WgXcQ?t=42",
        "https://www.youtube.com/embed/dQw4w9WgXcQ",
        "https://music.youtube.com/watch?v=dQw4w9WgXcQ&list=PLtest",
    ];

    for (i, url_str) in test_urls.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("url_parsing", i), url_str, |b, url_str| {
            b.iter(|| {
                let url = Url::parse(url_str).unwrap();
                black_box(url)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("query_extraction", i),
            url_str,
            |b, url_str| {
                b.iter(|| {
                    let url = Url::parse(url_str).unwrap();
                    let query_pairs: Vec<(String, String)> = url
                        .query_pairs()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    black_box(query_pairs)
                })
            },
        );
    }

    group.finish();
}

fn bench_regex_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_operations");

    // Common regex patterns used in YouTube source processing
    let video_id_pattern = regex::Regex::new(r"[a-zA-Z0-9_-]{11}").unwrap();
    let playlist_id_pattern = regex::Regex::new(r"PL[a-zA-Z0-9_-]{32}").unwrap();
    let timestamp_pattern = regex::Regex::new(r"[?&]t=(\d+)").unwrap();

    let test_strings = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s",
        "Check out this video: dQw4w9WgXcQ",
        "Playlist: https://www.youtube.com/playlist?list=PLtest123456789012345678901234567890",
        "Video at timestamp: https://youtu.be/dQw4w9WgXcQ?t=123",
    ];

    for (i, test_str) in test_strings.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("video_id_extraction", i),
            test_str,
            |b, text| {
                b.iter(|| {
                    let matches: Vec<_> = video_id_pattern.find_iter(text).collect();
                    black_box(matches)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("playlist_id_extraction", i),
            test_str,
            |b, text| {
                b.iter(|| {
                    let matches: Vec<_> = playlist_id_pattern.find_iter(text).collect();
                    black_box(matches)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("timestamp_extraction", i),
            test_str,
            |b, text| {
                b.iter(|| {
                    let captures = timestamp_pattern.captures(text);
                    black_box(captures)
                })
            },
        );
    }

    group.finish();
}

fn bench_json_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_operations");

    let sample_json = r#"{
        "videoDetails": {
            "videoId": "dQw4w9WgXcQ",
            "title": "Rick Astley - Never Gonna Give You Up",
            "lengthSeconds": "212",
            "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
            "isLiveContent": false
        },
        "streamingData": {
            "formats": [
                {
                    "itag": 18,
                    "url": "https://example.com/video.mp4",
                    "mimeType": "video/mp4; codecs=\"avc1.42001E, mp4a.40.2\"",
                    "bitrate": 568000,
                    "width": 640,
                    "height": 360
                }
            ]
        }
    }"#;

    group.bench_function("json_parsing", |b| {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(sample_json).unwrap();
            black_box(parsed)
        })
    });

    group.bench_function("json_field_access", |b| {
        let parsed: serde_json::Value = serde_json::from_str(sample_json).unwrap();
        b.iter(|| {
            let video_id = parsed["videoDetails"]["videoId"].as_str();
            let title = parsed["videoDetails"]["title"].as_str();
            let formats = &parsed["streamingData"]["formats"];
            black_box((video_id, title, formats))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_cipher_creation,
    bench_basic_cipher_operations,
    bench_url_operations,
    bench_regex_operations,
    bench_json_operations
);
criterion_main!(benches);
