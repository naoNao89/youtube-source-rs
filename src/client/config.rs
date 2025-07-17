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
        }
    }

    pub fn android() -> Self {
        Self {
            name: "ANDROID".to_string(),
            version: "19.50.37".to_string(),
            user_agent: "com.google.android.youtube/19.50.37 (Linux; U; Android 14; en_US; SM-G998B) gzip".to_string(),
            api_key: Some("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8".to_string()),
            client_name: "ANDROID".to_string(),
            client_version: "19.50.37".to_string(),
            platform: "MOBILE".to_string(),
            os_name: "Android".to_string(),
            os_version: "14".to_string(),
            visitor_data: None,
            po_token: None,
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

    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), self.user_agent.clone());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Origin".to_string(), "https://www.youtube.com".to_string());
        headers.insert("Referer".to_string(), "https://www.youtube.com/".to_string());
        
        if let Some(api_key) = &self.api_key {
            headers.insert("X-Goog-Api-Key".to_string(), api_key.clone());
        }
        
        headers
    }
}
