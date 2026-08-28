use serde::{Deserialize, Serialize};

use crate::pipeline::operation::QualityMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub preset_type: PresetType,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub video: Option<VideoPreset>,
    #[serde(default)]
    pub audio: Option<AudioPreset>,
    #[serde(default)]
    pub image: Option<ImagePreset>,
    #[serde(default)]
    pub container: Option<String>,
    #[serde(default)]
    pub builtin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresetType {
    Video,
    Audio,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPreset {
    pub codec: String,
    #[serde(default)]
    pub quality: Option<QualityMode>,
    #[serde(default)]
    pub fps: Option<f64>,
    #[serde(default)]
    pub extra_args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioPreset {
    pub codec: String,
    #[serde(default)]
    pub bitrate: Option<String>,
    #[serde(default)]
    pub sample_rate: Option<u32>,
    #[serde(default)]
    pub channels: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePreset {
    pub format: String,
    #[serde(default)]
    pub quality: Option<u8>,
    #[serde(default)]
    pub max_width: Option<u32>,
    #[serde(default)]
    pub max_height: Option<u32>,
}
