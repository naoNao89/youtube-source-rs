use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub name: String,
    pub version: String,
    pub user_agent: String,
    pub api_key: Option<String>,
    pub client_name: String,
    pub client_version: String,
    pub platform: String,
    pub os_name: String,
    pub os_version: String,
    pub visitor_data: Option<String>,
    pub po_token: Option<String>,
    // Additional fields for mobile and TV clients
    pub android_sdk_version: Option<u32>,
    pub device_make: Option<String>,
    pub device_model: Option<String>,
    pub third_party_embed_url: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::web()
    }
}

impl ClientConfig {
    pub fn web() -> Self {
        Self {
            name: "WEB".to_string(),
            version: "2.20241217.01.00".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "WEB".to_string(),
            client_version: "2.20241217.01.00".to_string(),
            platform: "DESKTOP".to_string(),
            os_name: "Windows".to_string(),
            os_version: "10.0".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: None,
            device_model: None,
            third_party_embed_url: None,
        }
    }

    /// Mobile Web client configuration - migrated from MWeb.java
    pub fn mobile_web() -> Self {
        Self {
            name: "MWEB".to_string(),
            version: "2.20240726.11.00".to_string(),
            user_agent: "Mozilla/5.0 (Linux; Android 11; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "MWEB".to_string(),
            client_version: "2.20240726.11.00".to_string(),
            platform: "MOBILE".to_string(),
            os_name: "Android".to_string(),
            os_version: "11".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: Some(30),
            device_make: Some("Samsung".to_string()),
            device_model: Some("SM-G973F".to_string()),
            third_party_embed_url: None,
        }
    }

    pub fn web_embedded() -> Self {
        Self {
            name: "WEB_EMBEDDED_PLAYER".to_string(),
            version: "1.20241217.01.00".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "WEB_EMBEDDED_PLAYER".to_string(),
            client_version: "1.20241217.01.00".to_string(),
            platform: "DESKTOP".to_string(),
            os_name: "Windows".to_string(),
            os_version: "10.0".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: None,
            device_model: None,
            third_party_embed_url: None,
        }
    }

    /// iOS client configuration - migrated from Ios.java
    pub fn ios() -> Self {
        Self {
            name: "IOS".to_string(),
            version: "19.09.3".to_string(),
            user_agent:
                "com.google.ios.youtube/19.09.3 (iPhone14,3; U; CPU iOS 15_6 like Mac OS X)"
                    .to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "IOS".to_string(),
            client_version: "19.09.3".to_string(),
            platform: "MOBILE".to_string(),
            os_name: "iOS".to_string(),
            os_version: "15.6".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: Some("Apple".to_string()),
            device_model: Some("iPhone14,3".to_string()),
            third_party_embed_url: None,
        }
    }

    /// TV HTML5 client configuration - migrated from Tv.java
    pub fn tv_html5() -> Self {
        Self {
            name: "TVHTML5".to_string(),
            version: "7.20250319.10.00".to_string(),
            user_agent: "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "TVHTML5".to_string(),
            client_version: "7.20250319.10.00".to_string(),
            platform: "TV".to_string(),
            os_name: "Cobalt".to_string(),
            os_version: "Version".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: None,
            device_model: None,
            third_party_embed_url: None,
        }
    }

    /// TV HTML5 Embedded client configuration - migrated from TvHtml5Embedded.java
    pub fn tv_html5_embedded() -> Self {
        Self {
            name: "TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string(),
            version: "2.0".to_string(),
            user_agent: "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string(),
            client_version: "2.0".to_string(),
            platform: "TV".to_string(),
            os_name: "Cobalt".to_string(),
            os_version: "Version".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: None,
            device_model: None,
            third_party_embed_url: Some("https://www.youtube.com".to_string()),
        }
    }

    pub fn android() -> Self {
        Self {
            name: "ANDROID".to_string(),
            version: "19.50.37".to_string(),
            user_agent:
                "com.google.android.youtube/19.50.37 (Linux; U; Android 14; en_US; SM-G998B) gzip"
                    .to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "ANDROID".to_string(),
            client_version: "19.50.37".to_string(),
            platform: "MOBILE".to_string(),
            os_name: "Android".to_string(),
            os_version: "14".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: Some(34),
            device_make: Some("Samsung".to_string()),
            device_model: Some("SM-G998B".to_string()),
            third_party_embed_url: None,
        }
    }

    pub fn music() -> Self {
        Self {
            name: "WEB_REMIX".to_string(),
            version: "1.20241217.01.00".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
            api_key: Some("AIzaSyC9XL3ZjWddXya6X74dJoCTL-WEYFDNX30".to_string()),
            client_name: "WEB_REMIX".to_string(),
            client_version: "1.20241217.01.00".to_string(),
            platform: "DESKTOP".to_string(),
            os_name: "Windows".to_string(),
            os_version: "10.0".to_string(),
            visitor_data: None,
            po_token: None,
            android_sdk_version: None,
            device_make: None,
            device_model: None,
            third_party_embed_url: None,
        }
    }

    pub fn to_context(&self) -> serde_json::Value {
        serde_json::json!({
            "client": {
                "clientName": self.client_name,
                "clientVersion": self.client_version,
                "platform": self.platform,
                "osName": self.os_name,
                "osVersion": self.os_version,
                "visitorData": self.visitor_data,
            }
        })
    }

    /// Convert to context JSON for API requests
    pub fn to_context_json(&self) -> serde_json::Value {
        let mut context = serde_json::json!({
            "client": {
                "clientName": self.client_name,
                "clientVersion": self.client_version,
                "platform": self.platform,
                "osName": self.os_name,
                "osVersion": self.os_version,
            }
        });

        // Add visitor data if available
        if let Some(visitor_data) = &self.visitor_data {
            context["client"]["visitorData"] = serde_json::Value::String(visitor_data.clone());
        }

        // Add user context
        context["user"] = serde_json::json!({
            "lockedSafetyMode": false
        });

        // Add request context
        context["request"] = serde_json::json!({
            "useSsl": true,
            "internalExperimentFlags": []
        });

        context
    }

    /// Get API key for requests
    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Get playback context (for player requests)
    pub fn get_playback_context(&self) -> Option<serde_json::Value> {
        // Basic playback context - can be extended for specific clients
        Some(serde_json::json!({
            "contentPlaybackContext": {
                "html5Preference": "HTML5_PREF_WANTS",
                "lactThreshold": -1,
                "referer": "https://www.youtube.com/",
                "signatureTimestamp": 19834
            }
        }))
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), self.user_agent.clone());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Origin".to_string(), "https://www.youtube.com".to_string());
        headers.insert(
            "Referer".to_string(),
            "https://www.youtube.com/".to_string(),
        );

        if let Some(api_key) = &self.api_key {
            headers.insert("X-Goog-Api-Key".to_string(), api_key.clone());
        }

        headers
    }
}
