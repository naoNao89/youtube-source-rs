use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpContextFilter {
    user_agent: String,
    default_headers: HashMap<String, String>,
}

impl HttpContextFilter {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "User-Agent".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        );
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());

        Self {
            user_agent: "youtube-source-rs/1.0".to_string(),
            default_headers: headers,
        }
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.default_headers
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.default_headers.insert(key, value);
    }

    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
        self.default_headers.insert("User-Agent".to_string(), self.user_agent.clone());
    }
}
