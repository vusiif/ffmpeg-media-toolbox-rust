use std::time::Duration;

#[derive(Debug, Clone)]
pub struct VideoStream {
    pub index: u32,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: Option<f64>,
    pub pixel_format: Option<String>,
    pub bit_rate: Option<u64>,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct AudioStream {
    pub index: u32,
    pub codec: String,
    pub sample_rate: u32,
    pub channels: u32,
    pub channel_layout: Option<String>,
    pub bit_rate: Option<u64>,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct SubtitleStream {
    pub index: u32,
    pub codec: String,
}
