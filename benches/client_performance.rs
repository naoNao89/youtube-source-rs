use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::runtime::Runtime;
use youtube_source_rs::client::{AndroidClient, IosClient, MusicClient, TvClient, WebClient};
use youtube_source_rs::{Client, YoutubeAudioSourceManager, YoutubeSourceOptions};

fn bench_client_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("client_creation");

    group.bench_function("android_client", |b| {
        b.iter(|| {
            let client = AndroidClient::new();
            black_box(client)
        })
    });

    group.bench_function("web_client", |b| {
        b.iter(|| {
            let client = WebClient::new().unwrap();
            black_box(client)
        })
    });

    group.bench_function("music_client", |b| {
        b.iter(|| {
            let client = MusicClient::new();
            black_box(client)
        })
    });

    group.bench_function("ios_client", |b| {
        b.iter(|| {
            let client = IosClient::new();
            black_box(client)
        })
    });

    group.bench_function("tv_client", |b| {
        b.iter(|| {
            let client = TvClient::new();
            black_box(client)
        })
    });

    group.finish();
}

fn bench_manager_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("manager_creation");

    group.bench_function("default_manager", |b| {
        b.iter(|| {
            let manager = YoutubeAudioSourceManager::new();
            black_box(manager)
        })
    });

    group.bench_function("manager_with_options", |b| {
        b.iter(|| {
            let options = YoutubeSourceOptions::default()
                .set_allow_search(true)
                .set_allow_direct_video_ids(true);
            let manager = YoutubeAudioSourceManager::with_options(options);
            black_box(manager)
        })
    });

    group.finish();
}

fn bench_client_capabilities(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("client_capabilities");

    let clients = vec![
        ("android", Box::new(AndroidClient::new()) as Box<dyn Client>),
        (
            "web",
            Box::new(WebClient::new().unwrap()) as Box<dyn Client>,
        ),
        ("music", Box::new(MusicClient::new()) as Box<dyn Client>),
        ("ios", Box::new(IosClient::new()) as Box<dyn Client>),
        ("tv", Box::new(TvClient::new()) as Box<dyn Client>),
    ];

    for (name, client) in clients {
        group.bench_with_input(
            BenchmarkId::new("can_handle_video", name),
            &client,
            |b, client| {
                b.iter(|| {
                    let result =
                        client.can_handle_request("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
                    black_box(result)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("can_handle_playlist", name),
            &client,
            |b, client| {
                b.iter(|| {
                    let result =
                        client.can_handle_request("https://www.youtube.com/playlist?list=PLtest");
                    black_box(result)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("supports_oauth", name),
            &client,
            |b, client| {
                b.iter(|| {
                    let result = client.supports_oauth();
                    black_box(result)
                })
            },
        );
    }

    group.finish();
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

    let client = AndroidClient::new();

    for url in test_urls {
        group.bench_with_input(BenchmarkId::new("can_handle", url), url, |b, url| {
            b.iter(|| {
                let result = client.can_handle_request(url);
                black_box(result)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_client_creation,
    bench_manager_creation,
    bench_client_capabilities,
    bench_url_parsing
);
criterion_main!(benches);
