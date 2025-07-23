use criterion::{black_box, criterion_group, criterion_main, Criterion};
use youtube_source_rs::{YoutubeAudioSourceManager, YoutubeSourceOptions};

fn bench_manager_creation(c: &mut Criterion) {
    c.bench_function("manager_creation", |b| {
        b.iter(|| {
            let manager = YoutubeAudioSourceManager::new();
            black_box(manager)
        })
    });
}

fn bench_manager_with_options(c: &mut Criterion) {
    c.bench_function("manager_with_options", |b| {
        b.iter(|| {
            let options = YoutubeSourceOptions::default()
                .set_allow_search(true)
                .set_allow_direct_video_ids(true);
            let manager = YoutubeAudioSourceManager::with_options(options);
            black_box(manager)
        })
    });
}

fn bench_url_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_parsing");

    let test_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "https://www.youtube.com/playlist?list=PLtest123",
        "https://music.youtube.com/watch?v=dQw4w9WgXcQ",
        "ytsearch:never gonna give you up",
        "ytmsearch:rick astley",
    ];

    for (i, url) in test_urls.iter().enumerate() {
        group.bench_with_input(format!("url_{}", i), url, |b, url| {
            b.iter(|| {
                // Simple URL validation benchmark
                let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
                let is_search = url.starts_with("ytsearch:") || url.starts_with("ytmsearch:");
                black_box((is_youtube, is_search))
            })
        });
    }

    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    let test_strings = vec![
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "1234567890abcdefghijklmnop",
        "zyxwvutsrqponmlkjihgfedcba",
    ];

    for (i, string) in test_strings.iter().enumerate() {
        group.bench_with_input(format!("reverse_{}", i), string, |b, s| {
            b.iter(|| {
                let reversed: String = s.chars().rev().collect();
                black_box(reversed)
            })
        });

        group.bench_with_input(format!("uppercase_{}", i), string, |b, s| {
            b.iter(|| {
                let upper = s.to_uppercase();
                black_box(upper)
            })
        });
    }

    group.finish();
}

fn bench_regex_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_operations");

    // Common regex patterns used in YouTube source processing
    let video_id_pattern = regex::Regex::new(r"[a-zA-Z0-9_-]{11}").unwrap();
    let playlist_id_pattern = regex::Regex::new(r"PL[a-zA-Z0-9_-]{32}").unwrap();

    let test_strings = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s",
        "Check out this video: dQw4w9WgXcQ",
        "Playlist: https://www.youtube.com/playlist?list=PLtest123456789012345678901234567890",
        "Video at timestamp: https://youtu.be/dQw4w9WgXcQ?t=123",
    ];

    for (i, test_str) in test_strings.iter().enumerate() {
        group.bench_with_input(format!("video_id_{}", i), test_str, |b, text| {
            b.iter(|| {
                let matches: Vec<_> = video_id_pattern.find_iter(text).collect();
                black_box(matches)
            })
        });

        group.bench_with_input(format!("playlist_id_{}", i), test_str, |b, text| {
            b.iter(|| {
                let matches: Vec<_> = playlist_id_pattern.find_iter(text).collect();
                black_box(matches)
            })
        });
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
    bench_manager_creation,
    bench_manager_with_options,
    bench_url_parsing,
    bench_string_operations,
    bench_regex_operations,
    bench_json_operations
);
criterion_main!(benches);
