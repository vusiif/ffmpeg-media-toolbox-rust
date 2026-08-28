use std::path::PathBuf;

use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::jobs::job::{ConvertRequest, JobId, JobRequest, ProbeRequest};
use media_core::jobs::queue::{JobQueue, QueueStats};
use media_core::preset::builtin::builtin_presets;
use media_core::preset::preset::Preset;

pub struct AppState {
    pub ffmpeg_valid: bool,
    pub files: Vec<PathBuf>,
    pub selected_preset: Option<Preset>,
    pub presets: Vec<Preset>,
    pub queue: JobQueue,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        let ffmpeg_valid = FfmpegLocator::new().is_ok();
        let presets = builtin_presets();

        Self {
            ffmpeg_valid,
            files: Vec::new(),
            selected_preset: presets.first().cloned(),
            presets,
            queue: JobQueue::new(),
            last_error: None,
        }
    }

    pub fn add_files(&mut self, paths: Vec<PathBuf>) {
        for p in paths {
            if !self.files.contains(&p) {
                self.files.push(p);
            }
        }
    }

    pub fn add_directory(&mut self, path: PathBuf) {
        if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_file() && !self.files.contains(&p) {
                        self.files.push(p);
                    }
                }
            }
        }
    }

    pub fn start_convert_job(&mut self, input: PathBuf) {
        let preset = match &self.selected_preset {
            Some(p) => p,
            None => {
                self.last_error = Some("No preset selected".into());
                return;
            }
        };

        let ext = preset.container.as_deref().unwrap_or("mp4");
        let output = input.with_extension(ext);

        let operation = media_core::pipeline::operation::ConvertOperation {
            video_codec: preset.video.as_ref().map(|v| v.codec.clone()),
            audio_codec: preset.audio.as_ref().map(|a| a.codec.clone()),
            format: preset.container.clone(),
            quality: preset.video.as_ref().and_then(|v| v.quality.clone()),
            extra_args: preset
                .video
                .as_ref()
                .map(|v| v.extra_args.clone())
                .unwrap_or_default(),
        };

        let request = JobRequest::Convert(ConvertRequest {
            input,
            output,
            operation,
            dry_run: false,
        });

        self.queue.enqueue(request);
    }

    pub fn cancel_job(&mut self, id: &str) {
        let job_id = JobId(id.to_string());
        self.queue.cancel(&job_id);
    }

    pub fn remove_job(&mut self, id: &str) {
        let job_id = JobId(id.to_string());
        self.queue.remove(&job_id);
    }

    pub fn retry_job(&mut self, id: &str) {
        let job_id = JobId(id.to_string());
        if let Some(job) = self.queue.get_mut(&job_id) {
            if matches!(job.status, media_core::jobs::job::JobStatus::Failed(_)) {
                job.status = media_core::jobs::job::JobStatus::Pending;
                job.progress = None;
                job.error_message = None;
                job.start_time = None;
                job.end_time = None;
                job.stderr_tail.clear();
            }
        }
    }

    pub fn clear_completed_jobs(&mut self) {
        self.queue.clear_completed();
    }

    pub fn queue_stats(&self) -> QueueStats {
        self.queue.stats()
    }

    pub fn start_probe_job(&mut self, input: PathBuf) {
        let request = JobRequest::Probe(ProbeRequest {
            input,
            json_output: false,
        });
        self.queue.enqueue(request);
    }

    pub fn remove_file(&mut self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
        }
    }

    pub fn clear_files(&mut self) {
        self.files.clear();
    }
}
