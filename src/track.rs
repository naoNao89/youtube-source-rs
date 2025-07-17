use std::time::Duration;
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackInfo {
    pub title: String,
    pub author: String,
    pub duration: Duration,
    pub video_id: String,
    pub is_stream: bool,
    pub uri: Url,
    pub thumbnail: Option<String>,
    pub artwork_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct YoutubeAudioTrack {
    pub info: AudioTrackInfo,
    pub source_manager: std::sync::Arc<crate::YoutubeAudioSourceManager>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatInfo {
    WebmOpus,
    WebmVorbis,
    Mp4AacLc,
    WebmVideoVorbis,
    Mp4VideoAacLc,
}

impl FormatInfo {
    pub fn mime_type(&self) -> &'static str {
        match self {
            FormatInfo::WebmOpus | FormatInfo::WebmVorbis => "audio/webm",
            FormatInfo::Mp4AacLc => "audio/mp4",
            FormatInfo::WebmVideoVorbis => "video/webm",
            FormatInfo::Mp4VideoAacLc => "video/mp4",
        }
    }

    pub fn codec(&self) -> &'static str {
        match self {
            FormatInfo::WebmOpus => "opus",
            FormatInfo::WebmVorbis | FormatInfo::WebmVideoVorbis => "vorbis",
            FormatInfo::Mp4AacLc | FormatInfo::Mp4VideoAacLc => "mp4a.40.2",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamFormat {
    pub info: Option<FormatInfo>,
    pub content_type: String,
    pub itag: u32,
    pub bitrate: u64,
    pub content_length: u64,
    pub audio_channels: u64,
    pub url: Url,
    pub n_parameter: Option<String>,
    pub signature: Option<String>,
    pub signature_key: Option<String>,
    pub is_default_audio_track: bool,
    pub is_drc: bool,
}

#[derive(Debug, Clone)]
pub struct TrackFormats {
    pub formats: Vec<StreamFormat>,
    pub player_script_url: Url,
}

impl TrackFormats {
    pub fn new(formats: Vec<StreamFormat>, player_script_url: Url) -> Self {
        Self {
            formats,
            player_script_url,
        }
    }

    pub fn get_best_format(&self) -> crate::Result<&StreamFormat> {
        let mut best_format: Option<&StreamFormat> = None;

        for format in &self.formats {
            if !format.is_default_audio_track {
                continue;
            }

            if Self::is_better_format(format, best_format) {
                best_format = Some(format);
            }
        }

        best_format.ok_or_else(|| {
            let available_types: Vec<String> = self.formats
                .iter()
                .map(|f| f.content_type.clone())
                .collect();
            crate::YoutubeError::Parse(format!(
                "No supported audio streams available, available types: {}",
                available_types.join(", ")
            ))
        })
    }

    fn is_better_format(format: &StreamFormat, other: Option<&StreamFormat>) -> bool {
        if format.info.is_none() {
            return false;
        }
        
        match other {
            None => true,
            Some(other_format) => {
                // Prefer higher bitrate
                format.bitrate > other_format.bitrate
            }
        }
    }
}
