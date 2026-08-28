use std::path::PathBuf;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ffmpeg::command::FfmpegCommand;
use crate::pipeline::operation::{ConvertOperation, ImageOperation};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(pub String);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobRequest {
    Convert(ConvertRequest),
    Image(ImageRequest),
    Probe(ProbeRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertRequest {
    pub input: PathBuf,
    pub output: PathBuf,
    pub operation: ConvertOperation,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    pub input: PathBuf,
    pub output: PathBuf,
    pub operations: Vec<ImageOperation>,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeRequest {
    pub input: PathBuf,
    pub json_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Preparing,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct JobProgress {
    pub percentage: Option<f64>,
    pub fps: Option<f64>,
    pub speed: Option<f64>,
    pub frame: Option<u64>,
}

pub struct Job {
    pub id: JobId,
    pub request: JobRequest,
    pub status: JobStatus,
    pub progress: Option<JobProgress>,
    pub command: Option<FfmpegCommand>,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub error_message: Option<String>,
    pub stderr_tail: Vec<String>,
}

impl Job {
    pub fn new(request: JobRequest) -> Self {
        Self {
            id: JobId::new(),
            request,
            status: JobStatus::Pending,
            progress: None,
            command: None,
            start_time: None,
            end_time: None,
            error_message: None,
            stderr_tail: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        match &self.request {
            JobRequest::Convert(req) => req
                .input
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            JobRequest::Image(req) => req
                .input
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            JobRequest::Probe(req) => req
                .input
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Completed | JobStatus::Failed(_) | JobStatus::Cancelled
        )
    }

    pub fn workload(&self) -> Workload {
        match &self.request {
            JobRequest::Convert(_) => Workload::VideoCpu,
            JobRequest::Image(_) => Workload::Image,
            JobRequest::Probe(_) => Workload::Probe,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Workload {
    Image,
    Audio,
    VideoCpu,
    VideoGpu,
    Probe,
}

impl Workload {
    pub fn default_concurrency(self) -> usize {
        match self {
            Self::Image => 4,
            Self::Audio => 2,
            Self::VideoCpu => 1,
            Self::VideoGpu => 2,
            Self::Probe => 4,
        }
    }
}
