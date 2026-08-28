use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaOperation {
    Convert(ConvertOperation),
    Resize(ResizeOperation),
    Crop(CropOperation),
    Rotate(RotateOperation),
    Flip(FlipOperation),
    Trim(TrimOperation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageOperation {
    Crop(CropOperation),
    Resize(ResizeOperation),
    Rotate(RotateOperation),
    Flip(FlipOperation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOperation {
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub format: Option<String>,
    pub quality: Option<QualityMode>,
    pub extra_args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityMode {
    Crf(u8),
    Bitrate(u64),
    Lossless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOperation {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub mode: ResizeMode,
    pub keep_aspect: bool,
    pub prevent_upscale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResizeMode {
    Exact,
    Fit,
    Fill,
    Percentage(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropOperation {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RotateOperation {
    CW90,
    CCW90,
    R180,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FlipOperation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimOperation {
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub fast: bool,
}

impl ImageOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Crop(_) => "crop",
            Self::Resize(_) => "resize",
            Self::Rotate(_) => "rotate",
            Self::Flip(_) => "flip",
        }
    }
}
