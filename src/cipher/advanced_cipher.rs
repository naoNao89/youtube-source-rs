use super::{ExtractedCipher, JavaScriptEngine, ScriptParser};
use crate::{Result, StreamFormat};
use std::time::Instant;
use url::Url;

/// Advanced signature cipher that uses JavaScript execution for real cipher operations
#[derive(Debug, Clone)]
pub struct AdvancedSignatureCipher {
    pub extracted_cipher: ExtractedCipher,
    js_engine: JavaScriptEngine,
}

impl AdvancedSignatureCipher {
    /// Create a new advanced signature cipher from a player script
    pub fn from_script(script: &str) -> Result<Self> {
        let extracted_cipher = ScriptParser::extract_cipher_from_script(script)?;
        let js_engine = JavaScriptEngine::new()?;

        Ok(Self {
            extracted_cipher,
            js_engine,
        })
    }

    /// Create from pre-extracted cipher information
    pub fn from_extracted_cipher(extracted_cipher: ExtractedCipher) -> Result<Self> {
        let js_engine = JavaScriptEngine::new()?;

        Ok(Self {
            extracted_cipher,
            js_engine,
        })
    }

    /// Decipher a URL by applying signature and N parameter transformations
    pub fn decipher_url(&self, format: &StreamFormat) -> Result<Url> {
        let mut url = format.url.clone();

        // Handle signature decryption if present
        if let Some(signature) = &format.signature {
            let deciphered_signature = self.decipher_signature(signature)?;
            url = self.build_url_with_signature(format, &deciphered_signature)?;
        }

        // Handle N parameter transformation if present
        if let Some(n_param) = &format.n_parameter {
            let transformed_n = self.transform_n_parameter(n_param)?;
            url = self.add_n_parameter_to_url(url, &transformed_n)?;
        }

        Ok(url)
    }

    /// Decipher a signature using JavaScript execution
    pub fn decipher_signature(&self, signature: &str) -> Result<String> {
        let start_time = Instant::now();

        // Prepare the JavaScript environment
        let script = format!(
            "{}\n{}\n{}",
            self.extracted_cipher.global_vars,
            self.extracted_cipher.sig_actions,
            self.extracted_cipher.sig_function
        );

        // Execute the signature function
        let result = self
            .js_engine
            .execute_cipher_function(&script, "sig", signature)?;

        // Log performance
        let execution_time = start_time.elapsed();
        if execution_time.as_millis() > 50 {
            log::warn!(
                "Signature decryption took {}ms, target is <50ms",
                execution_time.as_millis()
            );
        } else {
            log::debug!(
                "Signature decrypted in {}ms: '{}' -> '{}'",
                execution_time.as_millis(),
                signature,
                result
            );
        }

        Ok(result)
    }

    /// Transform N parameter using JavaScript execution
    pub fn transform_n_parameter(&self, n_param: &str) -> Result<String> {
        let start_time = Instant::now();

        // Prepare the JavaScript environment
        let script = format!(
            "{}\n{}",
            self.extracted_cipher.global_vars, self.extracted_cipher.n_function
        );

        // Execute the N parameter function
        let result = self
            .js_engine
            .execute_n_transform_function(&script, "n", n_param)?;

        // Log performance and validate result
        let execution_time = start_time.elapsed();
        if execution_time.as_millis() > 50 {
            log::warn!(
                "N parameter transformation took {}ms, target is <50ms",
                execution_time.as_millis()
            );
        }

        // Validate N parameter transformation
        if result == n_param {
            log::warn!(
                "N parameter transformation returned same value: '{n_param}' -> '{result}' (possible short-circuit)"
            );
        } else if result.starts_with("enhanced_except_")
            || result.ends_with(&format!("_w8_{n_param}"))
        {
            log::warn!(
                "N parameter transformation failed with exception pattern: '{n_param}' -> '{result}'"
            );
        } else {
            log::debug!(
                "N parameter transformed in {}ms: '{}' -> '{}'",
                execution_time.as_millis(),
                n_param,
                result
            );
        }

        Ok(result)
    }

    /// Build URL with deciphered signature
    fn build_url_with_signature(&self, format: &StreamFormat, signature: &str) -> Result<Url> {
        let mut url = format.url.clone();

        if let Some(signature_key) = &format.signature_key {
            // Add the deciphered signature to the URL
            let mut query_pairs: Vec<(String, String)> = url
                .query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            query_pairs.push((signature_key.clone(), signature.to_string()));

            // Rebuild URL with new query parameters
            url.set_query(None);
            let query_string = query_pairs
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");

            url.set_query(Some(&query_string));
        }

        Ok(url)
    }

    /// Add N parameter to URL
    fn add_n_parameter_to_url(&self, mut url: Url, n_value: &str) -> Result<Url> {
        let mut query_pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        // Remove existing n parameter if present
        query_pairs.retain(|(k, _)| k != "n");

        // Add the new n parameter
        query_pairs.push(("n".to_string(), n_value.to_string()));

        // Rebuild URL with new query parameters
        url.set_query(None);
        let query_string = query_pairs
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        url.set_query(Some(&query_string));

        Ok(url)
    }

    /// Get cipher timestamp for caching purposes
    pub fn get_timestamp(&self) -> &str {
        &self.extracted_cipher.timestamp
    }

    /// Test the cipher with sample data
    pub fn test_cipher(&self) -> Result<()> {
        // Test signature decryption with a sample signature
        let test_signature = "abcdefghijklmnopqrstuvwxyz0123456789";
        let result = self.decipher_signature(test_signature)?;

        if result != test_signature {
            log::info!("Cipher test passed: signature transformation working");
        } else {
            log::warn!("Cipher test warning: signature unchanged (may be identity function)");
        }

        // Test N parameter transformation
        let test_n = "abc123def456";
        let n_result = self.transform_n_parameter(test_n)?;

        if n_result != test_n {
            log::info!("N parameter test passed: transformation working");
        } else {
            log::warn!("N parameter test warning: value unchanged (may be identity function)");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cipher::ExtractedCipher;

    fn create_test_cipher() -> ExtractedCipher {
        ExtractedCipher {
            timestamp: "19834".to_string(),
            global_vars: r#"var a = "abcdefghijklmnopqrstuvwxyz0123456789".split("");"#.to_string(),
            sig_actions: r#"var b = {
                reverse: function(c) { c.reverse(); },
                swap: function(c, d) { var e = c[0]; c[0] = c[d % c.length]; c[d % c.length] = e; },
                splice: function(c, d) { c.splice(0, d); }
            };"#
            .to_string(),
            sig_function: r#"var sig = function(c) {
                var d = c.split('');
                b.reverse(d);
                b.swap(d, 1);
                b.splice(d, 2);
                return d.join('');
            }"#
            .to_string(),
            n_function: r#"var n = function(c) {
                return 'yt_' + c.split('').reverse().join('');
            }"#
            .to_string(),
            raw_script: "test script".to_string(),
        }
    }

    #[test]
    fn test_advanced_cipher_creation() {
        let cipher_info = create_test_cipher();
        let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info);
        assert!(cipher.is_ok());
    }

    #[test]
    fn test_signature_decryption() {
        let cipher_info = create_test_cipher();
        let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info).unwrap();

        let result = cipher.decipher_signature("abcdef");
        assert!(result.is_ok());

        let decrypted = result.unwrap();
        // Should be different from input due to transformations
        assert_ne!(decrypted, "abcdef");
    }

    #[test]
    fn test_n_parameter_transformation() {
        let cipher_info = create_test_cipher();
        let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info).unwrap();

        let result = cipher.transform_n_parameter("test123");
        assert!(result.is_ok());

        let transformed = result.unwrap();
        assert_eq!(transformed, "yt_321tset");
    }

    #[test]
    fn test_cipher_testing() {
        let cipher_info = create_test_cipher();
        let cipher = AdvancedSignatureCipher::from_extracted_cipher(cipher_info).unwrap();

        let result = cipher.test_cipher();
        assert!(result.is_ok());
    }
}
