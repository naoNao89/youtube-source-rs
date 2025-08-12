# YouTube Source Rust

[![CI](https://github.com/lavalink-devs/youtube-source-rs/workflows/CI/badge.svg)](https://github.com/lavalink-devs/youtube-source-rs/actions)
[![codecov](https://codecov.io/gh/lavalink-devs/youtube-source-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/lavalink-devs/youtube-source-rs)
[![Crates.io](https://img.shields.io/crates/v/youtube-source-rs.svg)](https://crates.io/crates/youtube-source-rs)
[![Documentation](https://docs.rs/youtube-source-rs/badge.svg)](https://docs.rs/youtube-source-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust rewrite of the YouTube source manager for Lavalink.

This source aims to provide robustness by leveraging multiple InnerTube clients
for requests. Where one client fails, another will try to load the request.
Which clients are used is entirely configurable.

## 🚀 Features

- **High Performance**: Built with Rust for maximum performance and memory safety
- **Multiple Client Support**: Leverages various YouTube InnerTube clients for reliability
- **Comprehensive Testing**: >90% code coverage with extensive integration tests
- **CI/CD Pipeline**: Automated testing, building, and deployment
- **Cross-Platform**: Supports Linux, macOS, and Windows
- **Lavalink Integration**: Drop-in replacement for the Java YouTube source
- **OAuth Support**: Authenticate with YouTube for enhanced reliability
- **poToken Support**: Use Proof of Origin tokens to bypass bot detection

## Table of Contents
- [🚀 Features](#-features)
- [📦 Library](#library)
  - Information about the Rust library and usage.
- [🔌 Plugin](#plugin)
  - Information about the Lavalink plugin and usage.
- [🛠️ Development](#️-development)
  - Development setup and testing information.
- [📊 Available Clients](#available-clients)
  - Information about the clients provided by `youtube-source-rust`, as well as their advantages/disadvantages.
- [🔐 Using OAuth tokens](#using-oauth-tokens)
  - Information on using OAuth tokens with `youtube-source-rust`.
- [🎫 Using a poToken](#using-a-potoken)
  - Information on using a `poToken` with `youtube-source-rust`.
- [🌐 REST Routes (`plugin` only)](#rest-routes-plugin-only)
  - Information on the REST routes provided by the `youtube-source-rust` plugin.
- [📈 Migration Information](#migration-from-java-implementation)
  - Information on migrating from the Java `youtube-source` implementation.
- [💬 Additional Support](#additional-support)
  - For everything else.

## 📦 Library
This module provides the base source manager, which can be used with any
Rust audio streaming application or as a standalone YouTube audio extraction library.

<details>
<summary>Using in Cargo.toml:</summary>

```toml
[dependencies]
# Replace VERSION with the current version as shown by the Releases tab
youtube-source-rust = "VERSION"
```

</details>

Example usage:

```rust
use youtube_source_rust::YoutubeAudioSourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let youtube = YoutubeAudioSourceManager::new();

    // Optionally, you may instantiate the source with custom options, such as toggling use of searching, and clients.
    let youtube = YoutubeAudioSourceManager::with_options_and_clients(
        YoutubeSourceOptions::default().set_allow_search(true),
        vec![
            Box::new(MusicClient::new()),
            Box::new(WebClient::new()?),
            Box::new(AndroidClient::new()),
        ]
    );

    // Load a video
    let video_url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let audio_item = youtube.load_item(video_url).await?;

    Ok(())
}
```

You may also implement the `Client` trait to support additional InnerTube clients. There are several
base implementations to make this easier, notably, `MusicClient` (for `music.youtube.com` InnerTube clients),
`WebClient` (for youtube.com innertube clients) and other specialized clients for different platforms.

Support for IP rotation can be achieved using custom HTTP client configuration:
```rust
use youtube_source_rust::{YoutubeAudioSourceManager, YoutubeSourceOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = YoutubeSourceOptions::default()
        .set_proxy_url(Some("http://your-proxy:8080".to_string()));

    let youtube = YoutubeAudioSourceManager::with_options(options);

    Ok(())
}
```

## 🔌 Plugin
This module serves as the plugin for use with [Lavalink](https://github.com/lavalink-devs/Lavalink).

To use this plugin with Lavalink, you must declare the dependency.

<details>
<summary>Using with Lavalink v3:</summary>

```yaml
lavalink:
  plugins:
    # Replace VERSION with the current version as shown by the Releases tab or a long commit hash for snapshots.
    - dependency: "dev.lavalink.youtube:youtube-source-rust:VERSION"
      repository: "https://maven.lavalink.dev/releases" # use https://maven.lavalink.dev/snapshots if you want to use a snapshot version.
```

</details>

<details>
<summary>Using with Lavalink v4:</summary>

```yaml
lavalink:
  plugins:
    # Replace VERSION with the current version as shown by the Releases tab or a long commit hash for snapshots.
    - dependency: "dev.lavalink.youtube:youtube-source-rust:VERSION"
      snapshot: false # Set to true if you want to use a snapshot version.
```

</details>

Configuring the plugin:
> [!IMPORTANT]
> You must make sure to disable the built-in YouTube source like so:
```yaml
lavalink:
  server:
    sources:
      youtube: false
```

> [!NOTE]
> Existing options, such as `ratelimit` and `youtubePlaylistLoadLimit` will be picked up automatically by the plugin,
> so these don't need changing.
>
```yaml
plugins:
  youtube:
    enabled: true # Whether this source can be used.
    allowSearch: true # Whether "ytsearch:" and "ytmsearch:" can be used.
    allowDirectVideoIds: true # Whether just video IDs can match. If false, only complete URLs will be loaded.
    allowDirectPlaylistIds: true # Whether just playlist IDs can match. If false, only complete URLs will be loaded.
    # The clients to use for track loading. See below for a list of valid clients.
    # Clients are queried in the order they are given (so the first client is queried first and so on...)
    clients:
      - MUSIC
      - ANDROID_VR
      - WEB
      - WEBEMBEDDED
```

## 🛠️ Development

### Prerequisites

- Rust 1.70.0 or later
- Docker and Docker Compose (for integration testing)
- Node.js 18+ (for test bot)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/lavalink-devs/youtube-source-rs.git
cd youtube-source-rs

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Testing

We provide comprehensive testing infrastructure:

#### Unit and Integration Tests
```bash
# Run all tests
cargo test --all-features

# Run specific test categories
cargo test unit_tests
cargo test integration_tests
cargo test cipher_tests
cargo test utils_tests

# Run with mock testing
cargo test --features mock-testing
```

#### Lavalink Integration Testing

We provide a complete Docker-based testing environment:

```bash
cd test-infrastructure

# Start test environment
./run-tests.sh start

# Run all integration tests
./run-tests.sh test

# Check service status
./run-tests.sh status

# View logs
./run-tests.sh logs

# Stop environment
./run-tests.sh stop
```

**Test Environment Includes:**
- Lavalink v4 (port 2333)
- Lavalink v3 (port 2334)
- Redis (port 6379)
- Prometheus (port 9090)
- Grafana (port 3000)
- Automated test bot

#### Performance Benchmarks

```bash
# Run performance benchmarks
cargo bench

# View benchmark results
open target/criterion/report/index.html
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Security audit
cargo audit

# Generate documentation
cargo doc --open
```

### Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development guidelines, CI/CD information, and contribution process.

### Advanced Options
```yaml
    # The below section of the config allows setting specific options for each client, such as the requests they will handle.
    # If an option, or client, is unspecified, then the default option value/client values will be used instead.
    # If a client is configured, but is not registered above, the options for that client will be ignored.
    # WARNING!: THE BELOW CONFIG IS FOR ILLUSTRATION PURPOSES. DO NOT COPY OR USE THIS WITHOUT
    # WARNING!: UNDERSTANDING WHAT IT DOES. MISCONFIGURATION WILL HINDER YOUTUBE-SOURCE'S ABILITY TO WORK PROPERLY.

    # Write the names of clients as they are specified under the heading "Available Clients".
    clientOptions:
      WEB:
        # Example: Disabling a client's playback capabilities.
        playback: false
        videoLoading: false # Disables loading of videos for this client. A client may still be used for playback even if this is set to 'false'.
      WEBEMBEDDED:
        # Example: Configuring a client to exclusively be used for video loading and playback.
        playlistLoading: false # Disables loading of playlists and mixes.
        searching: false # Disables the ability to search for videos.


## 📊 Available Clients

Currently, the following clients are available for use:

| Identifier        | Opus Formats | OAuth | Age-restriction Support | Playback Support | Metadata Support             | Additional Notes                                     |
|-------------------|--------------|-------|-------------------------|------------------|------------------------------|------------------------------------------------------|
| `MUSIC`           | No           | No    | No                      | No               | Search                       | YouTube music search support via `ytmsearch:` prefix |
| `WEB`             | Yes          | No    | No                      | Yes + Livestream | Video, Search, Playlist, Mix |                                                      |
| `MWEB`            | Yes          | No    | No                      | Yes + Livestream | Video, Search, Playlist, Mix |                                                      |
| `WEBEMBEDDED`     | Yes          | No    | Limited                 | Yes + Livestream | Video                        |                                                      |
| `ANDROID`         | Yes          | No    | No                      | Yes + Livestream | Video, Search, Playlist, Mix | Heavily restricted, frequently dysfunctional         |
| `ANDROID_MUSIC`   | Yes          | No    | No                      | Yes              | Video, Search, Mix           |                                                      |
| `ANDROID_VR`      | Yes          | No    | No                      | Yes + Livestream | Video, Search, Playlist, Mix |                                                      |
| `IOS`             | No           | No    | No                      | Yes + Livestream | Video, Search, Playlist, Mix |                                                      |
| `TV`              | Yes          | Yes   | With OAuth              | Yes + Livestream | None                         | Playback requires sign-in                            |
| `TVHTML5EMBEDDED` | Yes          | Yes   | With OAuth              | Yes + Livestream | Video, Search, Mix           | Playback requires sign-in                            |

> [!NOTE]
> Clients that do not return Opus formats will require transcoding.
> Livestreams do not yield Opus formats so will always require transcoding.


## 🔐 Using OAuth Tokens
You may notice that some requests are flagged by YouTube, causing an error message asking you to sign in to confirm you're not a bot.
With OAuth integration, you can request that `youtube-source-rust` use your account credentials to appear as a normal user, with varying degrees
of efficacy. **You do _not_ need to use `poToken` with OAuth.**

> [!WARNING]
> Similar to the `poToken` method, this is NOT a silver bullet solution, and worst case could get your account terminated!
> For this reason, it is advised that **you use burner accounts and NOT your primary!**.
> This method may also trigger ratelimit errors if used in a high traffic environment.
> USE WITH CAUTION!

> [!NOTE]
> You may need to set your log level for `youtube_source_rust::http::oauth` to `INFO`, to see additional information
> within your terminal regarding completing the OAuth flow.

> [!NOTE]
> If you do not have a refresh token, then do not supply one. The source will output your refresh token into your terminal upon
> successfully completing the OAuth flow at least **once**. If you do not see your token, you may need to configure your
> logging (see above note).

You can instruct `youtube-source-rust` to use OAuth with the following:

### Rust Library
```rust
use youtube_source_rust::YoutubeAudioSourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = YoutubeAudioSourceManager::new();

    // This will trigger an OAuth flow, where you will be instructed to head to YouTube's OAuth page and input a code.
    // This is safe, as it only uses YouTube's official OAuth flow. No tokens are seen or stored by us.
    source.use_oauth2(None, false).await?;

    // If you already have a refresh token, you can instruct the source to use it, skipping the OAuth flow entirely.
    // You can also set the `skip_initialization` parameter, which skips the OAuth flow. This should only be used
    // if you intend to supply a refresh token later on. You **must** either complete the OAuth flow or supply
    // a refresh token for OAuth integration to work.
    source.use_oauth2(Some("your refresh token".to_string()), true).await?;

    Ok(())
}
```

### Lavalink
```yaml
plugins:
  youtube:
    enabled: true
    oauth:
      # setting "enabled: true" is the bare minimum to get OAuth working.
      enabled: true

      # if you have a refresh token, you may set it below (make sure to uncomment the line to apply it).
      # setting a valid refresh token will skip the OAuth flow entirely. See above note on how to retrieve
      # your refreshToken.
      # refreshToken: "paste your refresh token here if applicable"

      # Set this if you don't want the OAuth flow to be triggered, if you intend to supply a refresh token later.
      # Initialization is skipped automatically if a valid refresh token is supplied. Leave this commented if you're
      # completing the OAuth flow for the first time/do not have a refresh token.
      # skipInitialization: true
```

### Passing an oauth token from your client
Another option to use oauth is by using oauth access tokens that are managed from your client. In this case your
bot/client provides LavaLink with the token to use when playing a track. To do this simply add the oauth access token
to a track's [userData](https://lavalink.dev/api/rest#track) field in a json format when updating the player to
play a track like:
```json
{
  "oauth-token": "access token to use"
}
```

## 🎫 Using a `poToken`
A `poToken`, also known as a "Proof of Origin Token" is a way to identify what requests originate from.
In YouTube's case, this is sent as a JavaScript challenge that browsers must evaluate, and send back the resolved
string. Typically, this challenge would remain unsolved for bots as more often than not, they don't simulate an entire
browser environment, instead only evaluating the minimum amount of JS required to do its job. Therefore, it's a reasonable
assumption that if the challenge is not fulfilled, the request origin is a bot.

To obtain a `poToken`, you can use https://github.com/iv-org/youtube-trusted-session-generator, by running the Python script
or the docker image. Both methods will print a `poToken` after a successful run, which you can supply to `youtube-source-rust`
to try and work around having automated requests blocked.


> [!NOTE]
> A `poToken` is not a silver bullet, and currently it only applies to requests made via the `WEB` & `WEBEMBEDDED` client.
> You do not need to specify a `poToken` if using OAuth, and vice versa.

Specifying the token is as simple as doing:

### Rust Library
```rust
// Web is youtube_source_rust::client::WebClient
use youtube_source_rust::YoutubeSource;

YoutubeSource::set_po_token_and_visitor_data("your po_token", "your visitor_data");
```

### Lavalink
```yaml
plugins:
  youtube:
    pot:
      token: "paste your po_token here"
      visitorData: "paste your visitor_data here"
```

## 🌐 REST Routes (`plugin` only)
### `POST` `/youtube`

Body:

> [!NOTE]
> You do not need to provide everything as it is shown.
> For example, you can specify just `refreshToken` and `skipInitialization`, or just `poToken` and `visitorData`.
> You do **not** need to use `poToken` with OAuth and vice versa.

```json
{
  "refreshToken": "your new refresh token",
  "skipInitialization": true,
  "poToken": "your po_token",
  "visitorData": "your visitor_data"
}
```

Response:

If the YouTube source is not enabled, or the `refreshToken` is invalid:
`500 - Internal Server Error`

Otherwise:
`204 - No Content`

### `GET` `/youtube`

Response:

If the YouTube source is not enabled:
`500 - Internal Server Error`

Otherwise:
```json
{
  "refreshToken": "your current refresh token, or null"
}
```

### `GET` `/youtube/stream/{videoId}`

Query parameters:

| Key          | Value Type | Required | Notes                                                                                                                                                                       |
|--------------|------------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| itag         | integer    | No       | The [itag](https://gist.github.com/AgentOak/34d47c65b1d28829bb17c24c04a0096f) of the desired format. If unspecified, youtube-source's default format selector will be used. |
| withClient   | string     | No       | The identifier of the client to use for streaming. Uses all clients if unspecified.                                                                                         |

Response:

If `videoId` could not be found or loaded, or the `itag` does not exist, or if no client supports format loading:
`400 - Bad Request`

Otherwise:
`200 - OK` accompanied by the selected format stream (audio or video). `Content-Type` header will be set appropriately.

### `GET` `/youtube/oauth/{refreshToken}`

Response:

If the `refreshToken` is invalid, expired, or cannot be processed:
`500 - Internal Server Error`

If the refresh process succeeds and a new access token is generated:
`200 - OK` accompanied by the new access token in JSON format.

Example response:
```json
{
  "access_token": "AccessToken",
  "expires_in": 69420,
  "scope": "used scope",
  "token_type": "type"
}
```



## 📈 Migration from Java Implementation

This client is intended as a direct replacement for the Java `YoutubeAudioSourceManager`,
which has been deprecated in a recent release of [Lavalink-Devs/Lavaplayer](https://github.com/lavalink-devs/lavaplayer).

When using `AudioSourceManagers.registerRemoteSources(AudioPlayerManager)`, Lavaplayer will register its own
deprecated `YoutubeAudioSourceManager`, which must be disabled.
Some versions of Lavaplayer may include an optional `excludeSources` parameter, allowing you to toggle the adding of the source.
If the version you are using does not support this, you will need to manually register each `AudioSourceManager` yourself.

First, create and register an instance of the supported `YoutubeAudioSourceManager` from the `youtube-source-rust` package.
```rust
use youtube_source_rust::YoutubeAudioSourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt_source_manager = YoutubeAudioSourceManager::new();
    // Register with your audio player manager
    Ok(())
}
```

In addition, there are a few significant changes to note:

- This source's class structure differs so if you had custom classes that you were initialising
  the source manager with (e.g. an overridden `YoutubeTrackDetailsLoader`), this **is not** compatible
  with this source manager.

## Versioning Policy
This project follows [Semantic Versioning](https://semver.org/), except in the case of [client](#available-clients) removal.
Typically, clients are not removed unless there is good reason, such as being deprecated, irreparably broken or removed from YouTube's client lifecycle.
In such scenarios, we anticipate that you have ceased usage of such clients prior to their removal, so do not expect any code breakage,
however we advise that you periodically check and keep your client list up to date due to this.

## 💬 Additional Support
If you need additional help with using this source, that's not covered here or in any of the issues,
[join our Discord server](https://discord.gg/ZW4s47Ppw4).
