use std::path::PathBuf;
use std::process::Command;

use crate::MediaError;

#[derive(Debug, Clone)]
pub struct FfmpegLocator {
    ffmpeg_path: PathBuf,
    ffprobe_path: PathBuf,
}

impl FfmpegLocator {
    pub fn new() -> Result<Self, MediaError> {
        let ffmpeg = Self::find_executable("ffmpeg")?;
        let ffprobe = Self::find_executable("ffprobe")?;
        Ok(Self {
            ffmpeg_path: ffmpeg,
            ffprobe_path: ffprobe,
        })
    }

    pub fn with_paths(ffmpeg: PathBuf, ffprobe: PathBuf) -> Result<Self, MediaError> {
        if !ffmpeg.exists() {
            return Err(MediaError::FfmpegNotFound);
        }
        if !ffprobe.exists() {
            return Err(MediaError::FfprobeNotFound);
        }
        Ok(Self {
            ffmpeg_path: ffmpeg,
            ffprobe_path: ffprobe,
        })
    }

    pub fn ffmpeg_path(&self) -> &PathBuf {
        &self.ffmpeg_path
    }

    pub fn ffprobe_path(&self) -> &PathBuf {
        &self.ffprobe_path
    }

    pub fn ffmpeg_version(&self) -> Result<String, MediaError> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output()
            .map_err(|_| MediaError::FfmpegNotFound)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let first_line = stdout.lines().next().unwrap_or("");
        Ok(first_line.to_string())
    }

    fn find_executable(name: &str) -> Result<PathBuf, MediaError> {
        let exe = if cfg!(target_os = "windows") {
            format!("{}.exe", name)
        } else {
            name.to_string()
        };

        if let Ok(path) = which(&exe) {
            return Ok(path);
        }

        let error = match name {
            "ffmpeg" => MediaError::FfmpegNotFound,
            "ffprobe" => MediaError::FfprobeNotFound,
            _ => MediaError::Other(format!("Executable not found: {}", name)),
        };
        Err(error)
    }
}

fn which(name: &str) -> Result<PathBuf, MediaError> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(target_os = "windows") { ';' } else { ':' };

    for dir in path_var.split(sep) {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(MediaError::Other(format!("{} not found in PATH", name)))
}
