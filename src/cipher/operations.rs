use crate::{Result, StreamFormat};
use url::Url;

#[derive(Debug, Clone)]
pub struct SignatureCipher {
    pub operations: Vec<CipherOperation>,
}

#[derive(Debug, Clone)]
pub enum CipherOperation {
    Reverse,
    Swap(usize),
    Slice(usize),
}

impl SignatureCipher {
    pub fn new(operations: Vec<CipherOperation>) -> Self {
        Self { operations }
    }

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

    fn decipher_signature(&self, signature: &str) -> Result<String> {
        let mut chars: Vec<char> = signature.chars().collect();

        for operation in &self.operations {
            match operation {
                CipherOperation::Reverse => {
                    chars.reverse();
                }
                CipherOperation::Swap(index) => {
                    if *index < chars.len() {
                        chars.swap(0, *index);
                    }
                }
                CipherOperation::Slice(index) => {
                    if *index < chars.len() {
                        chars = chars[*index..].to_vec();
                    }
                }
            }
        }

        Ok(chars.into_iter().collect())
    }

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

    /// Transform N parameter (throttling parameter)
    fn transform_n_parameter(&self, n_param: &str) -> Result<String> {
        // For now, implement a basic transformation
        // In a real implementation, this would use the actual N parameter transformation function
        // from the YouTube player script

        // Simple transformation: reverse the string and add a prefix
        let mut chars: Vec<char> = n_param.chars().collect();
        chars.reverse();
        let transformed = format!("yt_{}", chars.into_iter().collect::<String>());

        Ok(transformed)
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
}
