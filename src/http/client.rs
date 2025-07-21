use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    default_headers: HashMap<String, String>,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "User-Agent".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        );

        Self {
            client: reqwest::Client::new(),
            default_headers: headers,
        }
    }

    pub async fn get(&self, url: &str) -> crate::Result<reqwest::Response> {
        let mut request = self.client.get(url);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        Ok(request.send().await?)
    }

    pub async fn post(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> crate::Result<reqwest::Response> {
        let mut request = self.client.post(url);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        Ok(request.json(&body).send().await?)
    }
}
