use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// I/O utilities for the plugin system
///
/// Migrated from: `youtube-source-java/plugin/src/main/java/dev/lavalink/youtube/plugin/IOUtils.java`
///
/// Provides utility functions for file operations and data handling within the plugin.
pub struct IOUtils;

impl IOUtils {
    /// Read all bytes from a file
    pub fn read_file_to_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Read all text from a file as UTF-8
    pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let bytes = Self::read_file_to_bytes(path)?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Write bytes to a file
    pub fn write_bytes_to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        file.flush()?;
        Ok(())
    }

    /// Write text to a file as UTF-8
    pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
        Self::write_bytes_to_file(path, content.as_bytes())
    }

    /// Copy data from a reader to a writer
    pub fn copy<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
        let mut buffer = [0; 8192];
        let mut total = 0;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            writer.write_all(&buffer[..bytes_read])?;
            total += bytes_read as u64;
        }

        writer.flush()?;
        Ok(total)
    }

    /// Read all bytes from a reader
    pub fn read_all<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Read all text from a reader as UTF-8
    pub fn read_all_string<R: Read>(reader: &mut R) -> io::Result<String> {
        let bytes = Self::read_all(reader)?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Check if a file exists
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_file()
    }

    /// Check if a directory exists
    pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_dir()
    }

    /// Create directories recursively
    pub fn create_directories<P: AsRef<Path>>(path: P) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    /// Get file size in bytes
    pub fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len())
    }

    /// Delete a file if it exists
    pub fn delete_file<P: AsRef<Path>>(path: P) -> io::Result<bool> {
        let path = path.as_ref();
        if path.exists() {
            std::fs::remove_file(path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Delete a directory and all its contents
    pub fn delete_directory<P: AsRef<Path>>(path: P) -> io::Result<bool> {
        let path = path.as_ref();
        if path.exists() && path.is_dir() {
            std::fs::remove_dir_all(path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Detail message builder for error reporting
///
/// Migrated from: `youtube-source-java/common/src/main/java/dev/lavalink/youtube/polyfill/DetailMessageBuilder.java`
///
/// Provides utilities for building detailed error messages with context information.
pub struct DetailMessageBuilder {
    message: String,
    details: Vec<String>,
}

impl DetailMessageBuilder {
    /// Create a new detail message builder
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            details: Vec::new(),
        }
    }

    /// Add a detail to the message
    pub fn add_detail(mut self, detail: &str) -> Self {
        self.details.push(detail.to_string());
        self
    }

    /// Add a key-value detail
    pub fn add_key_value(mut self, key: &str, value: &str) -> Self {
        self.details.push(format!("{key}: {value}"));
        self
    }

    /// Add an optional detail (only if Some)
    pub fn add_optional_detail(mut self, detail: Option<&str>) -> Self {
        if let Some(detail) = detail {
            self.details.push(detail.to_string());
        }
        self
    }

    /// Add an optional key-value detail
    pub fn add_optional_key_value(mut self, key: &str, value: Option<&str>) -> Self {
        if let Some(value) = value {
            self.details.push(format!("{key}: {value}"));
        }
        self
    }

    /// Build the final message
    pub fn build(self) -> String {
        if self.details.is_empty() {
            self.message
        } else {
            format!("{}\nDetails:\n{}", self.message, self.details.join("\n"))
        }
    }

    /// Build the message with custom separator
    pub fn build_with_separator(self, separator: &str) -> String {
        if self.details.is_empty() {
            self.message
        } else {
            format!(
                "{}{}{}",
                self.message,
                separator,
                self.details.join(separator)
            )
        }
    }
}

impl std::fmt::Display for DetailMessageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().build())
    }
}

impl Clone for DetailMessageBuilder {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            details: self.details.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_all() {
        let data = b"Hello, World!";
        let mut cursor = Cursor::new(data);
        let result = IOUtils::read_all(&mut cursor).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_read_all_string() {
        let data = "Hello, World!";
        let mut cursor = Cursor::new(data.as_bytes());
        let result = IOUtils::read_all_string(&mut cursor).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_copy() {
        let data = b"Hello, World!";
        let mut reader = Cursor::new(data);
        let mut writer = Vec::new();

        let bytes_copied = IOUtils::copy(&mut reader, &mut writer).unwrap();
        assert_eq!(bytes_copied, data.len() as u64);
        assert_eq!(writer, data);
    }

    #[test]
    fn test_detail_message_builder() {
        let message = DetailMessageBuilder::new("Error occurred")
            .add_detail("First detail")
            .add_key_value("Key", "Value")
            .add_optional_detail(Some("Optional detail"))
            .add_optional_detail(None)
            .build();

        assert!(message.contains("Error occurred"));
        assert!(message.contains("First detail"));
        assert!(message.contains("Key: Value"));
        assert!(message.contains("Optional detail"));
    }

    #[test]
    fn test_detail_message_builder_empty() {
        let message = DetailMessageBuilder::new("Simple message").build();
        assert_eq!(message, "Simple message");
    }

    #[test]
    fn test_detail_message_builder_custom_separator() {
        let message = DetailMessageBuilder::new("Error")
            .add_detail("Detail 1")
            .add_detail("Detail 2")
            .build_with_separator(" | ");

        assert!(message.contains("Error | Detail 1 | Detail 2"));
    }
}
