use std::path::Path;
use std::process::Command;

use crate::media::MediaFile;
use crate::MediaError;

use super::locator::FfmpegLocator;

pub fn probe_file(locator: &FfmpegLocator, path: &Path) -> Result<MediaFile, MediaError> {
    if !path.exists() {
        return Err(MediaError::InputNotFound(path.to_path_buf()));
    }

    let output = Command::new(locator.ffprobe_path())
        .arg("-v")
        .arg("error")
        .arg("-of")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg("-show_chapters")
        .arg(path)
        .output()
        .map_err(|e| MediaError::Other(format!("Failed to run ffprobe: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(MediaError::Other(format!("ffprobe failed: {}", stderr)));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let raw: FfprobeOutput = serde_json::from_str(&json_str)?;

    Ok(raw.into_media_file(path))
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeOutput {
    format: Option<FfprobeFormat>,
    streams: Option<Vec<FfprobeStream>>,
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeFormat {
    filename: Option<String>,
    duration: Option<String>,
    size: Option<String>,
    bit_rate: Option<String>,
    format_name: Option<String>,
    #[serde(default)]
    tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeStream {
    index: Option<u32>,
    codec_type: Option<String>,
    codec_name: Option<String>,
    codec_long_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    sample_rate: Option<String>,
    channels: Option<u32>,
    channel_layout: Option<String>,
    bit_rate: Option<String>,
    duration: Option<String>,
    r_frame_rate: Option<String>,
    avg_frame_rate: Option<String>,
    pix_fmt: Option<String>,
    sample_fmt: Option<String>,
    #[serde(default)]
    tags: std::collections::HashMap<String, String>,
}

impl FfprobeOutput {
    fn into_media_file(self, path: &Path) -> MediaFile {
        use crate::media::{AudioStream, Metadata, SubtitleStream, VideoStream};
        use std::time::Duration;

        let format = self.format.unwrap_or(FfprobeFormat {
            filename: None,
            duration: None,
            size: None,
            bit_rate: None,
            format_name: None,
            tags: Default::default(),
        });

        let duration = format
            .duration
            .as_deref()
            .and_then(|d| d.parse::<f64>().ok())
            .map(Duration::from_secs_f64);

        let file_size = format
            .size
            .as_deref()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mut video_streams = Vec::new();
        let mut audio_streams = Vec::new();
        let mut subtitle_streams = Vec::new();

        for stream in self.streams.unwrap_or_default() {
            match stream.codec_type.as_deref() {
                Some("video") => {
                    video_streams.push(VideoStream {
                        index: stream.index.unwrap_or(0),
                        codec: stream.codec_name.clone().unwrap_or_default(),
                        width: stream.width.unwrap_or(0),
                        height: stream.height.unwrap_or(0),
                        fps: parse_frame_rate(stream.r_frame_rate.as_deref()),
                        pixel_format: stream.pix_fmt.clone(),
                        bit_rate: stream.bit_rate.as_deref().and_then(|s| s.parse().ok()),
                        duration: stream
                            .duration
                            .as_deref()
                            .and_then(|d| d.parse::<f64>().ok())
                            .map(Duration::from_secs_f64),
                    });
                }
                Some("audio") => {
                    audio_streams.push(AudioStream {
                        index: stream.index.unwrap_or(0),
                        codec: stream.codec_name.clone().unwrap_or_default(),
                        sample_rate: stream
                            .sample_rate
                            .as_deref()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0),
                        channels: stream.channels.unwrap_or(0),
                        channel_layout: stream.channel_layout.clone(),
                        bit_rate: stream.bit_rate.as_deref().and_then(|s| s.parse().ok()),
                        duration: stream
                            .duration
                            .as_deref()
                            .and_then(|d| d.parse::<f64>().ok())
                            .map(Duration::from_secs_f64),
                    });
                }
                Some("subtitle") => {
                    subtitle_streams.push(SubtitleStream {
                        index: stream.index.unwrap_or(0),
                        codec: stream.codec_name.clone().unwrap_or_default(),
                    });
                }
                _ => {}
            }
        }

        let metadata = Metadata {
            title: format.tags.get("title").cloned(),
            artist: format.tags.get("artist").cloned(),
            album: format.tags.get("album").cloned(),
            comment: format.tags.get("comment").cloned(),
            date: format.tags.get("date").cloned(),
            genre: format.tags.get("genre").cloned(),
        };

        MediaFile {
            path: path.to_path_buf(),
            file_size,
            duration,
            format_name: format.format_name.unwrap_or_default(),
            bit_rate: format.bit_rate.as_deref().and_then(|s| s.parse().ok()),
            video_streams,
            audio_streams,
            subtitle_streams,
            metadata,
        }
    }
}

fn parse_frame_rate(s: Option<&str>) -> Option<f64> {
    let s = s?;
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].parse().ok()?;
        let den: f64 = parts[1].parse().ok()?;
        if den != 0.0 {
            return Some(num / den);
        }
    }
    s.parse().ok()
}
