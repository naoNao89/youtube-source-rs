use serde::{Deserialize, Serialize};

/// PoToken configuration for bypassing YouTube bot detection
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/Pot.java`
/// 
/// A `poToken`, also known as a "Proof of Origin Token" is a way to identify what requests originate from.
/// In YouTube's case, this is sent as a JavaScript challenge that browsers must evaluate, and send back the resolved
/// string. This helps bypass bot detection mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct Pot {
    /// The PoToken string obtained from YouTube's JavaScript challenge
    pub token: Option<String>,
    
    /// The visitor data associated with the PoToken
    pub visitor_data: Option<String>,
}


impl Pot {
    /// Create a new Pot configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new Pot with token and visitor data
    pub fn with_token_and_visitor_data(token: String, visitor_data: String) -> Self {
        Self {
            token: Some(token),
            visitor_data: Some(visitor_data),
        }
    }
    
    /// Set the PoToken
    pub fn set_token(mut self, token: String) -> Self {
        self.token = if token.is_empty() { None } else { Some(token) };
        self
    }
    
    /// Set the visitor data
    pub fn set_visitor_data(mut self, visitor_data: String) -> Self {
        self.visitor_data = if visitor_data.is_empty() { None } else { Some(visitor_data) };
        self
    }
    
    /// Get the PoToken, returning None if empty or not set
    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref().filter(|t| !t.is_empty())
    }
    
    /// Get the visitor data, returning None if empty or not set
    pub fn get_visitor_data(&self) -> Option<&String> {
        self.visitor_data.as_ref().filter(|v| !v.is_empty())
    }
    
    /// Check if both token and visitor data are present
    pub fn is_complete(&self) -> bool {
        self.get_token().is_some() && self.get_visitor_data().is_some()
    }
    
    /// Check if the Pot configuration is valid (has at least a token)
    pub fn is_valid(&self) -> bool {
        self.get_token().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_pot() {
        let pot = Pot::default();
        assert!(pot.get_token().is_none());
        assert!(pot.get_visitor_data().is_none());
        assert!(!pot.is_complete());
        assert!(!pot.is_valid());
    }
    
    #[test]
    fn test_pot_with_token_and_visitor_data() {
        let pot = Pot::with_token_and_visitor_data(
            "test_token".to_string(),
            "test_visitor_data".to_string()
        );
        
        assert_eq!(pot.get_token(), Some(&"test_token".to_string()));
        assert_eq!(pot.get_visitor_data(), Some(&"test_visitor_data".to_string()));
        assert!(pot.is_complete());
        assert!(pot.is_valid());
    }
    
    #[test]
    fn test_empty_strings_treated_as_none() {
        let pot = Pot::new()
            .set_token("".to_string())
            .set_visitor_data("".to_string());
            
        assert!(pot.get_token().is_none());
        assert!(pot.get_visitor_data().is_none());
        assert!(!pot.is_valid());
    }
    
    #[test]
    fn test_token_only() {
        let pot = Pot::new().set_token("test_token".to_string());
        
        assert!(pot.get_token().is_some());
        assert!(pot.get_visitor_data().is_none());
        assert!(pot.is_valid());
        assert!(!pot.is_complete());
    }
}
