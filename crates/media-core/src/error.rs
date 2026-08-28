use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("FFmpeg executable was not found")]
    FfmpegNotFound,

    #[error("FFprobe executable was not found")]
    FfprobeNotFound,

    #[error("Input file does not exist: {0}")]
    InputNotFound(PathBuf),

    #[error("Unsupported encoder: {0}")]
    UnsupportedEncoder(String),

    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("FFmpeg process failed with exit code {0}: {1}")]
    ProcessFailed(i32, String),

    #[error("FFmpeg process was killed")]
    ProcessKilled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid preset: {0}")]
    InvalidPreset(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Pipeline error: {0}")]
    Pipeline(String),

    #[error("Job error: {0}")]
    Job(String),

    #[error("Output path error: {0}")]
    OutputPath(String),

    #[error("Cancelled by user")]
    Cancelled,

    #[error("{0}")]
    Other(String),
}
