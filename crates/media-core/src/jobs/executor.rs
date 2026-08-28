use std::sync::Arc;

use tokio::sync::mpsc;

use crate::ffmpeg::command::FfmpegCommand;
use crate::ffmpeg::locator::FfmpegLocator;
use crate::ffmpeg::process::FfmpegProcess;
use crate::ffmpeg::progress::FfmpegProgress;
use crate::jobs::job::{
    ConvertRequest, ImageRequest, Job, JobId, JobRequest, JobStatus, ProbeRequest,
};
use crate::pipeline::compiler::PipelineCompiler;
use crate::pipeline::operation::QualityMode;
use crate::MediaError;

#[derive(Debug, Clone)]
pub enum JobEvent {
    Started(JobId),
    Progress(JobId, JobProgressInfo),
    Completed(JobId),
    Failed(JobId, String),
    Cancelled(JobId),
}

#[derive(Debug, Clone)]
pub struct JobProgressInfo {
    pub percentage: Option<f64>,
    pub fps: Option<f64>,
    pub speed: Option<f64>,
    pub frame: Option<u64>,
}

pub struct JobExecutor {
    locator: Arc<FfmpegLocator>,
}

impl JobExecutor {
    pub fn new(locator: FfmpegLocator) -> Self {
        Self {
            locator: Arc::new(locator),
        }
    }

    pub async fn execute(
        &self,
        job: &mut Job,
        event_tx: &mpsc::UnboundedSender<JobEvent>,
    ) -> Result<(), MediaError> {
        event_tx
            .send(JobEvent::Started(job.id.clone()))
            .map_err(|e| MediaError::Other(e.to_string()))?;

        job.status = JobStatus::Running;
        job.start_time = Some(std::time::Instant::now());

        let result = match &job.request {
            JobRequest::Convert(req) => self.execute_convert(req, &job.id, event_tx).await,
            JobRequest::Image(req) => self.execute_image(req, &job.id, event_tx).await,
            JobRequest::Probe(req) => self.execute_probe(req).await,
        };

        job.end_time = Some(std::time::Instant::now());

        match result {
            Ok(()) => {
                job.status = JobStatus::Completed;
                event_tx
                    .send(JobEvent::Completed(job.id.clone()))
                    .map_err(|e| MediaError::Other(e.to_string()))?;
            }
            Err(e) => {
                let msg = e.to_string();
                job.status = JobStatus::Failed(msg.clone());
                job.error_message = Some(msg.clone());
                event_tx
                    .send(JobEvent::Failed(job.id.clone(), msg))
                    .map_err(|e| MediaError::Other(e.to_string()))?;
            }
        }

        Ok(())
    }

    async fn execute_convert(
        &self,
        req: &ConvertRequest,
        job_id: &JobId,
        event_tx: &mpsc::UnboundedSender<JobEvent>,
    ) -> Result<(), MediaError> {
        if !req.input.exists() {
            return Err(MediaError::InputNotFound(req.input.clone()));
        }

        let cmd = self.build_convert_command(req)?;

        if req.dry_run {
            tracing::info!("[dry-run] {}", cmd.to_command_string());
            return Ok(());
        }

        let mut process = FfmpegProcess::spawn(&cmd).await?;
        let id = job_id.clone();
        let tx = event_tx.clone();

        let state = process
            .wait_with_progress(|p: &FfmpegProgress| {
                let info = JobProgressInfo {
                    percentage: None,
                    fps: p.fps,
                    speed: p.speed,
                    frame: p.frame,
                };
                let _ = tx.send(JobEvent::Progress(id.clone(), info));
            })
            .await?;

        match state {
            crate::ffmpeg::process::ProcessState::Completed => Ok(()),
            crate::ffmpeg::process::ProcessState::Failed(code) => {
                let stderr = process.stderr_tail(10);
                Err(MediaError::ProcessFailed(code, stderr))
            }
            crate::ffmpeg::process::ProcessState::Killed => Err(MediaError::Cancelled),
            _ => Ok(()),
        }
    }

    async fn execute_image(
        &self,
        req: &ImageRequest,
        job_id: &JobId,
        event_tx: &mpsc::UnboundedSender<JobEvent>,
    ) -> Result<(), MediaError> {
        if !req.input.exists() {
            return Err(MediaError::InputNotFound(req.input.clone()));
        }

        let cmd = self.build_image_command(req)?;

        if req.dry_run {
            tracing::info!("[dry-run] {}", cmd.to_command_string());
            return Ok(());
        }

        let mut process = FfmpegProcess::spawn(&cmd).await?;
        let id = job_id.clone();
        let tx = event_tx.clone();

        let state = process
            .wait_with_progress(|p: &FfmpegProgress| {
                let info = JobProgressInfo {
                    percentage: None,
                    fps: p.fps,
                    speed: p.speed,
                    frame: p.frame,
                };
                let _ = tx.send(JobEvent::Progress(id.clone(), info));
            })
            .await?;

        match state {
            crate::ffmpeg::process::ProcessState::Completed => Ok(()),
            crate::ffmpeg::process::ProcessState::Failed(code) => {
                let stderr = process.stderr_tail(10);
                Err(MediaError::ProcessFailed(code, stderr))
            }
            crate::ffmpeg::process::ProcessState::Killed => Err(MediaError::Cancelled),
            _ => Ok(()),
        }
    }

    async fn execute_probe(&self, req: &ProbeRequest) -> Result<(), MediaError> {
        crate::ffmpeg::probe::probe_file(&self.locator, &req.input)?;
        Ok(())
    }

    fn build_convert_command(&self, req: &ConvertRequest) -> Result<FfmpegCommand, MediaError> {
        let mut cmd = FfmpegCommand::new(self.locator.ffmpeg_path().clone());
        cmd.arg("-i").arg(&req.input);

        if let Some(ref v) = req.operation.video_codec {
            cmd.arg("-c:v").arg(v);
        }
        if let Some(ref q) = req.operation.quality {
            match q {
                QualityMode::Crf(crf) => {
                    cmd.arg("-crf").arg(crf.to_string());
                }
                QualityMode::Bitrate(br) => {
                    cmd.arg("-b:v").arg(format!("{}k", br / 1000));
                }
                QualityMode::Lossless => {
                    cmd.arg("-crf").arg("0");
                }
            }
        }
        if let Some(ref a) = req.operation.audio_codec {
            cmd.arg("-c:a").arg(a);
        }
        if let Some(ref fmt) = req.operation.format {
            cmd.arg("-f").arg(fmt);
        }

        cmd.args(&req.operation.extra_args);
        cmd.arg("-y").arg(&req.output);

        Ok(cmd)
    }

    fn build_image_command(&self, req: &ImageRequest) -> Result<FfmpegCommand, MediaError> {
        let mut cmd = FfmpegCommand::new(self.locator.ffmpeg_path().clone());
        cmd.arg("-i").arg(&req.input);

        let fg = PipelineCompiler::compile_image_operations(&req.operations)?;
        if !fg.is_empty() {
            let filter_args = fg.to_filter_complex();
            cmd.args(filter_args);
        }

        cmd.arg("-y").arg(&req.output);

        Ok(cmd)
    }
}
