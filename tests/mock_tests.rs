#[cfg(feature = "mock-testing")]
mod mock_youtube_api_tests {
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_server() -> MockServer {
        MockServer::start().await
    }

    fn create_mock_video_response() -> serde_json::Value {
        json!({
            "videoDetails": {
                "videoId": "dQw4w9WgXcQ",
                "title": "Rick Astley - Never Gonna Give You Up (Official Video)",
                "lengthSeconds": "212",
                "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                "shortDescription": "The official video for Rick Astley's \"Never Gonna Give You Up\"",
                "viewCount": "1000000000",
                "author": "Rick Astley",
                "isLiveContent": false,
                "isPrivate": false,
                "allowRatings": true
            },
            "streamingData": {
                "expiresInSeconds": "21600",
                "formats": [
                    {
                        "itag": 18,
                        "url": "https://example.com/video.mp4?signature=test123",
                        "mimeType": "video/mp4; codecs=\"avc1.42001E, mp4a.40.2\"",
                        "bitrate": 568000,
                        "width": 640,
                        "height": 360,
                        "lastModified": "1234567890",
                        "contentLength": "50000000",
                        "quality": "medium",
                        "fps": 30,
                        "qualityLabel": "360p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 500000,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "212000",
                        "audioSampleRate": "44100",
                        "audioChannels": 2
                    }
                ],
                "adaptiveFormats": [
                    {
                        "itag": 140,
                        "url": "https://example.com/audio.m4a?signature=test456",
                        "mimeType": "audio/mp4; codecs=\"mp4a.40.2\"",
                        "bitrate": 128000,
                        "contentLength": "3400000",
                        "quality": "tiny",
                        "audioQuality": "AUDIO_QUALITY_MEDIUM",
                        "approxDurationMs": "212000",
                        "audioSampleRate": "44100",
                        "audioChannels": 2,
                        "loudnessDb": -14.5
                    }
                ]
            },
            "playabilityStatus": {
                "status": "OK",
                "playableInEmbed": true
            }
        })
    }

    fn create_mock_playlist_response() -> serde_json::Value {
        json!({
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "itemSectionRenderer": {
                                            "contents": [{
                                                "playlistVideoListRenderer": {
                                                    "contents": [
                                                        {
                                                            "playlistVideoRenderer": {
                                                                "videoId": "dQw4w9WgXcQ",
                                                                "thumbnail": {
                                                                    "thumbnails": [{
                                                                        "url": "https://example.com/thumb.jpg",
                                                                        "width": 120,
                                                                        "height": 90
                                                                    }]
                                                                },
                                                                "title": {
                                                                    "runs": [{
                                                                        "text": "Never Gonna Give You Up"
                                                                    }]
                                                                },
                                                                "shortBylineText": {
                                                                    "runs": [{
                                                                        "text": "Rick Astley"
                                                                    }]
                                                                },
                                                                "lengthText": {
                                                                    "simpleText": "3:32"
                                                                }
                                                            }
                                                        }
                                                    ]
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            },
            "metadata": {
                "playlistMetadataRenderer": {
                    "title": "Test Playlist",
                    "description": "A test playlist for mock testing"
                }
            }
        })
    }

    fn create_mock_search_response() -> serde_json::Value {
        json!({
            "contents": {
                "twoColumnSearchResultsRenderer": {
                    "primaryContents": {
                        "sectionListRenderer": {
                            "contents": [{
                                "itemSectionRenderer": {
                                    "contents": [
                                        {
                                            "videoRenderer": {
                                                "videoId": "dQw4w9WgXcQ",
                                                "thumbnail": {
                                                    "thumbnails": [{
                                                        "url": "https://example.com/thumb.jpg",
                                                        "width": 320,
                                                        "height": 180
                                                    }]
                                                },
                                                "title": {
                                                    "runs": [{
                                                        "text": "Rick Astley - Never Gonna Give You Up"
                                                    }]
                                                },
                                                "longBylineText": {
                                                    "runs": [{
                                                        "text": "Rick Astley"
                                                    }]
                                                },
                                                "lengthText": {
                                                    "simpleText": "3:32"
                                                },
                                                "viewCountText": {
                                                    "simpleText": "1B views"
                                                }
                                            }
                                        }
                                    ]
                                }
                            }]
                        }
                    }
                }
            }
        })
    }

    #[tokio::test]
    async fn test_mock_video_api() {
        let mock_server = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/youtubei/v1/player"))
            .and(query_param("key", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_mock_video_response()))
            .mount(&mock_server)
            .await;

        // Test that our mock server responds correctly
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}/youtubei/v1/player?key=test_key",
                mock_server.uri()
            ))
            .json(&json!({
                "videoId": "dQw4w9WgXcQ",
                "context": {
                    "client": {
                        "clientName": "ANDROID",
                        "clientVersion": "17.31.35"
                    }
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let json: serde_json::Value = response.json().await.unwrap();
        assert_eq!(json["videoDetails"]["videoId"], "dQw4w9WgXcQ");
        assert_eq!(
            json["videoDetails"]["title"],
            "Rick Astley - Never Gonna Give You Up (Official Video)"
        );
    }

    #[tokio::test]
    async fn test_mock_playlist_api() {
        let mock_server = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/youtubei/v1/browse"))
            .and(query_param("key", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_mock_playlist_response()))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}/youtubei/v1/browse?key=test_key",
                mock_server.uri()
            ))
            .json(&json!({
                "browseId": "VLPLtest123",
                "context": {
                    "client": {
                        "clientName": "WEB",
                        "clientVersion": "2.20230101.00.00"
                    }
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let json: serde_json::Value = response.json().await.unwrap();
        assert!(json["contents"].is_object());
        assert!(json["metadata"]["playlistMetadataRenderer"]["title"] == "Test Playlist");
    }

    #[tokio::test]
    async fn test_mock_search_api() {
        let mock_server = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/youtubei/v1/search"))
            .and(query_param("key", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_mock_search_response()))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}/youtubei/v1/search?key=test_key",
                mock_server.uri()
            ))
            .json(&json!({
                "query": "never gonna give you up",
                "context": {
                    "client": {
                        "clientName": "WEB",
                        "clientVersion": "2.20230101.00.00"
                    }
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let json: serde_json::Value = response.json().await.unwrap();
        assert!(json["contents"].is_object());
    }

    #[tokio::test]
    async fn test_mock_error_responses() {
        let mock_server = setup_mock_server().await;

        // Test 404 response
        Mock::given(method("POST"))
            .and(path("/youtubei/v1/player"))
            .and(query_param("key", "invalid_key"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": {
                    "code": 404,
                    "message": "Video not found"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}/youtubei/v1/player?key=invalid_key",
                mock_server.uri()
            ))
            .json(&json!({"videoId": "invalid"}))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_mock_rate_limiting() {
        let mock_server = setup_mock_server().await;

        // Test rate limiting response
        Mock::given(method("POST"))
            .and(path("/youtubei/v1/player"))
            .and(query_param("key", "rate_limited"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "60")
                    .set_body_json(json!({
                        "error": {
                            "code": 429,
                            "message": "Too many requests"
                        }
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}/youtubei/v1/player?key=rate_limited",
                mock_server.uri()
            ))
            .json(&json!({"videoId": "test"}))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 429);
        assert_eq!(response.headers().get("Retry-After").unwrap(), "60");
    }

    #[tokio::test]
    async fn test_mock_player_script() {
        let mock_server = setup_mock_server().await;

        let mock_script = r#"
            var a = {
                reverse: function(a) {
                    return a.reverse();
                },
                swap: function(a, b) {
                    var c = a[0];
                    a[0] = a[b % a.length];
                    a[b % a.length] = c;
                    return a;
                },
                slice: function(a, b) {
                    return a.slice(b);
                }
            };
            
            function decipherSignature(signature) {
                signature = signature.split('');
                signature = a.reverse(signature);
                signature = a.swap(signature, 1);
                signature = a.slice(signature, 2);
                return signature.join('');
            }
        "#;

        Mock::given(method("GET"))
            .and(path("/player.js"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(mock_script)
                    .insert_header("Content-Type", "application/javascript"),
            )
            .mount(&mock_server)
            .await;

        let response = reqwest::get(&format!("{}/player.js", mock_server.uri()))
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let script_content = response.text().await.unwrap();
        assert!(script_content.contains("decipherSignature"));
        assert!(script_content.contains("reverse"));
        assert!(script_content.contains("swap"));
        assert!(script_content.contains("slice"));
    }
}
