use url::Url;
use crate::{StreamFormat, Result, YoutubeError};

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
        if let Some(signature) = &format.signature {
            let deciphered_signature = self.decipher_signature(signature)?;
            self.build_url_with_signature(format, &deciphered_signature)
        } else {
            // No signature to decipher, return original URL
            Ok(format.url.clone())
        }
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
            let mut query_pairs: Vec<(String, String)> = url.query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            
            query_pairs.push((signature_key.clone(), signature.to_string()));
            
            // Rebuild URL with new query parameters
            url.set_query(None);
            let query_string = query_pairs
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            
            url.set_query(Some(&query_string));
        }

        Ok(url)
    }
}
