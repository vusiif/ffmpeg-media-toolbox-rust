use std::path::PathBuf;
use std::time::Duration;

use super::metadata::Metadata;
use super::stream::{AudioStream, SubtitleStream, VideoStream};

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub path: PathBuf,
    pub file_size: u64,
    pub duration: Option<Duration>,
    pub format_name: String,
    pub bit_rate: Option<u64>,
    pub video_streams: Vec<VideoStream>,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
    pub metadata: Metadata,
}

impl MediaFile {
    pub fn is_video(&self) -> bool {
        !self.video_streams.is_empty()
    }

    pub fn is_audio(&self) -> bool {
        !self.video_streams.is_empty() && !self.audio_streams.is_empty()
    }

    pub fn is_image(&self) -> bool {
        self.video_streams.len() == 1
            && self.audio_streams.is_empty()
            && self.duration.is_some_and(|d| d.as_secs_f64() < 1.0)
    }

    pub fn primary_video(&self) -> Option<&VideoStream> {
        self.video_streams.first()
    }

    pub fn primary_audio(&self) -> Option<&AudioStream> {
        self.audio_streams.first()
    }

    pub fn display_name(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    }
}
