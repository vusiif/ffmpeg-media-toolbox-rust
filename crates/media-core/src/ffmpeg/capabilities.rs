use std::collections::HashSet;
use std::process::Command;

use crate::MediaError;

use super::locator::FfmpegLocator;

#[derive(Debug, Clone, Default)]
pub struct FfmpegCapabilities {
    pub version: String,
    pub formats: HashSet<String>,
    pub codecs: HashSet<String>,
    pub encoders: HashSet<String>,
    pub decoders: HashSet<String>,
    pub filters: HashSet<String>,
    pub hardware_accelerators: HashSet<String>,
}

impl FfmpegCapabilities {
    pub fn detect(locator: &FfmpegLocator) -> Result<Self, MediaError> {
        let version = locator.ffmpeg_version()?;
        let formats = Self::parse_list(locator, "-formats")?;
        let codecs = Self::parse_list(locator, "-codecs")?;
        let encoders = Self::parse_list(locator, "-encoders")?;
        let decoders = Self::parse_list(locator, "-decoders")?;
        let filters = Self::parse_list(locator, "-filters")?;
        let hwaccels = Self::parse_hwaccels(locator)?;

        Ok(Self {
            version,
            formats,
            codecs,
            encoders,
            decoders,
            filters,
            hardware_accelerators: hwaccels,
        })
    }

    pub fn has_encoder(&self, name: &str) -> bool {
        self.encoders.contains(name)
    }

    pub fn has_decoder(&self, name: &str) -> bool {
        self.decoders.contains(name)
    }

    pub fn has_format(&self, name: &str) -> bool {
        self.formats.contains(name)
    }

    pub fn has_filter(&self, name: &str) -> bool {
        self.filters.contains(name)
    }

    pub fn has_hwaccel(&self, name: &str) -> bool {
        self.hardware_accelerators.contains(name)
    }

    fn parse_list(locator: &FfmpegLocator, flag: &str) -> Result<HashSet<String>, MediaError> {
        let output = Command::new(locator.ffmpeg_path())
            .arg(flag)
            .output()
            .map_err(|_| MediaError::FfmpegNotFound)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut set = HashSet::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("---") || trimmed.starts_with("File") {
                continue;
            }
            if let Some(name) = trimmed.split_whitespace().nth(1) {
                set.insert(name.to_string());
            }
        }

        Ok(set)
    }

    fn parse_hwaccels(locator: &FfmpegLocator) -> Result<HashSet<String>, MediaError> {
        let output = Command::new(locator.ffmpeg_path())
            .arg("-hwaccels")
            .output()
            .map_err(|_| MediaError::FfmpegNotFound)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut set = HashSet::new();
        let mut started = false;

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed == "Hardware acceleration methods:" {
                started = true;
                continue;
            }
            if started && !trimmed.is_empty() {
                set.insert(trimmed.to_string());
            }
        }

        Ok(set)
    }
}
