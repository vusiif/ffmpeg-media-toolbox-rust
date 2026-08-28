#[derive(Debug, Clone, Default)]
pub struct FfmpegProgress {
    pub frame: Option<u64>,
    pub out_time_us: Option<u64>,
    pub fps: Option<f64>,
    pub speed: Option<f64>,
    pub total_size: Option<u64>,
    pub dup_frames: Option<u64>,
    pub drop_frames: Option<u64>,
}

impl FfmpegProgress {
    pub fn parse_line(&mut self, line: &str) {
        let line = line.trim();
        if line.starts_with("progress") {
            return;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "frame" => self.frame = value.parse().ok(),
                "out_time_us" => self.out_time_us = value.parse().ok(),
                "fps" => self.fps = value.parse().ok(),
                "speed" => {
                    self.speed = value.trim_end_matches('x').parse().ok();
                }
                "total_size" => self.total_size = value.parse().ok(),
                "dup_frames" => self.dup_frames = value.parse().ok(),
                "drop_frames" => self.drop_frames = value.parse().ok(),
                _ => {}
            }
        }
    }

    pub fn percentage(&self, total_duration_us: Option<u64>) -> Option<f64> {
        let current = self.out_time_us?;
        let total = total_duration_us?;
        if total == 0 {
            return None;
        }
        Some((current as f64 / total as f64 * 100.0).min(100.0))
    }
}
