use super::{AdvancedSignatureCipher, ExtractedCipher, ScriptParser, SignatureCipher};
use crate::{Result, StreamFormat};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use url::Url;

#[derive(Debug, Clone)]
pub struct SignatureCipherManager {
    #[allow(clippy::arc_with_non_send_sync)]
    cached_scripts: Arc<RwLock<HashMap<String, CachedPlayerScript>>>,
    http_client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct CachedPlayerScript {
    pub script_content: String,
    pub cipher: SignatureCipher,
    pub advanced_cipher: Option<AdvancedSignatureCipher>,
    pub extracted_cipher: Option<ExtractedCipher>,
    pub cached_at: std::time::SystemTime,
}

impl Default for SignatureCipherManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SignatureCipherManager {
    pub fn new() -> Self {
        Self {
            #[allow(clippy::arc_with_non_send_sync)]
            cached_scripts: Arc::new(RwLock::new(HashMap::new())),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn resolve_format_url(
        &self,
        player_script_url: &Url,
        format: &StreamFormat,
    ) -> Result<Url> {
        // Try advanced cipher first, fallback to basic cipher
        if let Ok(advanced_cipher) = self.get_advanced_cipher(player_script_url).await {
            log::debug!("Using advanced JavaScript-based cipher for URL resolution");
            advanced_cipher.decipher_url(format)
        } else {
            log::warn!("Advanced cipher failed, falling back to basic cipher operations");
            let cipher = self.get_cipher(player_script_url).await?;
            cipher.decipher_url(format)
        }
    }

    pub async fn get_cipher(&self, player_script_url: &Url) -> Result<SignatureCipher> {
        let url_string = player_script_url.to_string();

        // Check cache first
        {
            let scripts = self.cached_scripts.read().unwrap();
            if let Some(cached) = scripts.get(&url_string) {
                // Check if cache is still valid (e.g., less than 1 hour old)
                if cached.cached_at.elapsed().unwrap_or_default().as_secs() < 3600 {
                    return Ok(cached.cipher.clone());
                }
            }
        }

        // Fetch and parse new script
        let script_content = self.fetch_player_script(player_script_url).await?;
        let cipher = self.parse_cipher_from_script(&script_content)?;

        // Cache the result
        {
            let mut scripts = self.cached_scripts.write().unwrap();
            scripts.insert(
                url_string,
                CachedPlayerScript {
                    script_content,
                    cipher: cipher.clone(),
                    advanced_cipher: None,
                    extracted_cipher: None,
                    cached_at: std::time::SystemTime::now(),
                },
            );
        }

        Ok(cipher)
    }

    pub async fn get_advanced_cipher(
        &self,
        player_script_url: &Url,
    ) -> Result<AdvancedSignatureCipher> {
        let url_string = player_script_url.to_string();

        // Check cache first
        {
            let scripts = self.cached_scripts.read().unwrap();
            if let Some(cached) = scripts.get(&url_string) {
                // Check if cache is still valid (e.g., less than 1 hour old)
                if cached.cached_at.elapsed().unwrap_or_default().as_secs() < 3600 {
                    if let Some(ref advanced_cipher) = cached.advanced_cipher {
                        log::debug!("Using cached advanced cipher for {url_string}");
                        return Ok(advanced_cipher.clone());
                    }
                }
            }
        }

        // Fetch and parse new script
        let script_content = self.fetch_player_script(player_script_url).await?;
        let extracted_cipher = ScriptParser::extract_cipher_from_script(&script_content)?;
        let advanced_cipher =
            AdvancedSignatureCipher::from_extracted_cipher(extracted_cipher.clone())?;

        // Test the cipher to ensure it's working
        advanced_cipher.test_cipher()?;

        // Cache the result
        {
            let mut scripts = self.cached_scripts.write().unwrap();

            // Update existing cache entry or create new one
            if let Some(cached) = scripts.get_mut(&url_string) {
                cached.advanced_cipher = Some(advanced_cipher.clone());
                cached.extracted_cipher = Some(extracted_cipher);
                cached.cached_at = std::time::SystemTime::now();
            } else {
                // Create basic cipher as fallback
                let basic_cipher = self.parse_cipher_from_script(&script_content)?;
                scripts.insert(
                    url_string.clone(),
                    CachedPlayerScript {
                        script_content,
                        cipher: basic_cipher,
                        advanced_cipher: Some(advanced_cipher.clone()),
                        extracted_cipher: Some(extracted_cipher),
                        cached_at: std::time::SystemTime::now(),
                    },
                );
            }
        }

        log::info!("Successfully created and cached advanced cipher for {url_string}");
        Ok(advanced_cipher)
    }

    async fn fetch_player_script(&self, url: &Url) -> Result<String> {
        let response = self.http_client.get(url.as_str()).send().await?;
        let content = response.text().await?;
        Ok(content)
    }

    fn parse_cipher_from_script(&self, _script: &str) -> Result<SignatureCipher> {
        // For now, implement a basic placeholder cipher that performs common operations
        // In a full implementation, this would parse the JavaScript to extract the actual operations
        use super::operations::CipherOperation;

        // Create a basic cipher with common operations found in YouTube player scripts
        // This is a simplified version - real implementation would parse the JavaScript
        let operations = vec![
            CipherOperation::Reverse,
            CipherOperation::Swap(1),
            CipherOperation::Slice(2),
        ];

        Ok(SignatureCipher::new(operations))
    }

    /// Get cache statistics for monitoring
    pub async fn get_cache_stats(&self) -> CacheStats {
        let scripts = self.cached_scripts.read().unwrap();
        let total_entries = scripts.len();
        let advanced_cipher_entries = scripts
            .values()
            .filter(|cached| cached.advanced_cipher.is_some())
            .count();
        let expired_entries = scripts
            .values()
            .filter(|cached| cached.cached_at.elapsed().unwrap_or_default().as_secs() >= 3600)
            .count();

        CacheStats {
            total_entries,
            advanced_cipher_entries,
            basic_cipher_entries: total_entries - advanced_cipher_entries,
            expired_entries,
        }
    }

    /// Clear expired cache entries
    pub async fn cleanup_cache(&self) {
        let mut scripts = self.cached_scripts.write().unwrap();
        let before_count = scripts.len();

        scripts.retain(|_, cached| cached.cached_at.elapsed().unwrap_or_default().as_secs() < 3600);

        let after_count = scripts.len();
        if before_count > after_count {
            log::info!(
                "Cleaned up {} expired cache entries",
                before_count - after_count
            );
        }
    }

    /// Force refresh a specific player script
    pub async fn refresh_script(&self, player_script_url: &Url) -> Result<()> {
        let url_string = player_script_url.to_string();

        // Remove from cache
        {
            let mut scripts = self.cached_scripts.write().unwrap();
            scripts.remove(&url_string);
        }

        // Force re-fetch
        let _ = self.get_advanced_cipher(player_script_url).await?;
        log::info!("Refreshed player script cache for {url_string}");

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub advanced_cipher_entries: usize,
    pub basic_cipher_entries: usize,
    pub expired_entries: usize,
}
