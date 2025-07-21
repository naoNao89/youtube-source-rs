use serde::{Deserialize, Serialize};
use log::{info, warn, debug};
use crate::YoutubeSource;

/// Plugin information and version checking
/// 
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/PluginInfo.java`
pub struct PluginInfo;

#[derive(Debug, Deserialize, Serialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    draft: Option<bool>,
    prerelease: Option<bool>,
}

impl PluginInfo {
    /// Check for new releases on GitHub
    pub async fn check_for_new_release() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let current_version = YoutubeSource::VERSION;
        
        if current_version == "Unknown" {
            debug!("Cannot compare versions - current version is unknown");
            return Ok(());
        }
        
        let current_version = Self::parse_version(current_version)?;
        
        match Self::fetch_latest_release().await {
            Ok(Some((latest_version, release_url))) => {
                if latest_version > current_version {
                    info!(
                        "********************************************\n\
                         YOUTUBE-SOURCE-RUST VERSION {} AVAILABLE\n\
                         {}\n\
                         Update to ensure the YouTube source remains operational!\n\
                         ********************************************",
                        Self::format_version(&latest_version),
                        release_url
                    );
                }
            }
            Ok(None) => {
                debug!("No releases found or unable to determine latest version");
            }
            Err(e) => {
                warn!("Failed to check for new releases: {e}");
            }
        }
        
        Ok(())
    }
    
    /// Fetch the latest release from GitHub
    async fn fetch_latest_release() -> Result<Option<(Version, String)>, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://api.github.com/repos/lavalink-devs/youtube-source/releases";
        
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "youtube-source-rust")
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(format!("GitHub API request failed: {}", response.status()).into());
        }
        
        let releases: Vec<GitHubRelease> = response.json().await?;
        
        let mut latest_version: Option<Version> = None;
        let mut latest_url: Option<String> = None;
        
        for release in releases {
            // Skip drafts and prereleases
            if release.draft.unwrap_or(false) || release.prerelease.unwrap_or(false) {
                continue;
            }
            
            if let Ok(version) = Self::parse_version(&release.tag_name) {
                if latest_version.is_none() || version > *latest_version.as_ref().unwrap() {
                    latest_version = Some(version);
                    latest_url = Some(release.html_url);
                }
            }
        }
        
        match (latest_version, latest_url) {
            (Some(version), Some(url)) => Ok(Some((version, url))),
            _ => Ok(None),
        }
    }
    
    /// Parse a version string into a comparable format
    fn parse_version(version_str: &str) -> Result<Version, Box<dyn std::error::Error + Send + Sync>> {
        // Remove 'v' prefix if present
        let version_str = version_str.strip_prefix('v').unwrap_or(version_str);
        
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 2 {
            return Err("Invalid version format".into());
        }
        
        let major = parts[0].parse::<u32>()?;
        let minor = parts[1].parse::<u32>()?;
        let patch = if parts.len() > 2 {
            parts[2].parse::<u32>().unwrap_or(0)
        } else {
            0
        };
        
        Ok(Version { major, minor, patch })
    }
    
    /// Format a version for display
    fn format_version(version: &Version) -> String {
        format!("{}.{}.{}", version.major, version.minor, version.patch)
    }
    
    /// Get the current plugin version
    pub fn get_version() -> &'static str {
        YoutubeSource::VERSION
    }
    
    /// Get plugin information
    pub fn get_info() -> PluginInformation {
        PluginInformation {
            name: "YouTube Source Rust".to_string(),
            version: Self::get_version().to_string(),
            description: "High-performance Rust implementation of YouTube audio source for Lavalink".to_string(),
            author: "YouTube Source Rust Team".to_string(),
            repository: "https://github.com/lavalink-devs/youtube-source-rust".to_string(),
        }
    }
}

/// Simple version representation for comparison
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Plugin information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInformation {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub repository: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_version() {
        let version = PluginInfo::parse_version("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        
        let version = PluginInfo::parse_version("v2.0.1").unwrap();
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 1);
        
        let version = PluginInfo::parse_version("1.5").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 5);
        assert_eq!(version.patch, 0);
    }
    
    #[test]
    fn test_version_comparison() {
        let v1 = PluginInfo::parse_version("1.2.3").unwrap();
        let v2 = PluginInfo::parse_version("1.2.4").unwrap();
        let v3 = PluginInfo::parse_version("1.3.0").unwrap();
        let v4 = PluginInfo::parse_version("2.0.0").unwrap();
        
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
    }
    
    #[test]
    fn test_format_version() {
        let version = Version { major: 1, minor: 2, patch: 3 };
        assert_eq!(PluginInfo::format_version(&version), "1.2.3");
    }
    
    #[test]
    fn test_get_info() {
        let info = PluginInfo::get_info();
        assert_eq!(info.name, "YouTube Source Rust");
        assert!(!info.version.is_empty());
        assert!(!info.description.is_empty());
    }
}
