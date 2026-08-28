use std::path::PathBuf;

use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::jobs::executor::JobProgressInfo;
use media_core::jobs::job::{ConvertRequest, JobRequest};
use media_core::preset::builtin::builtin_presets;
use media_core::preset::preset::Preset;

use crate::i18n::{Lang, Language};

pub struct AppState {
    pub ffmpeg_valid: bool,
    pub ffmpeg_path: Option<String>,
    pub ffprobe_path: Option<String>,
    pub ffmpeg_version: Option<String>,
    pub files: Vec<PathBuf>,
    pub selected_preset: Option<Preset>,
    pub presets: Vec<Preset>,
    pub last_error: Option<String>,
    pub lang: Lang,
    pub jobs: Vec<DisplayJob>,
    next_display_id: u64,
    pub batch_recursive: bool,
    pub batch_filter: String,
}

pub struct DisplayJob {
    pub id: String,
    pub name: String,
    pub status: DisplayJobStatus,
    pub progress: Option<JobProgressInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl DisplayJobStatus {
    pub fn label(&self, lang: &Lang) -> String {
        use crate::i18n::Key;
        match self {
            Self::Pending => lang.t(Key::Pending).to_string(),
            Self::Running => lang.t(Key::Running).to_string(),
            Self::Completed => lang.t(Key::Done).to_string(),
            Self::Failed => lang.t(Key::Failed).to_string(),
            Self::Cancelled => lang.t(Key::Cancelled).to_string(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let presets = builtin_presets();
        let (ffmpeg_valid, ffmpeg_path, ffprobe_path, ffmpeg_version) = detect_ffmpeg();

        Self {
            ffmpeg_valid,
            ffmpeg_path,
            ffprobe_path,
            ffmpeg_version,
            files: Vec::new(),
            selected_preset: presets.first().cloned(),
            presets,
            last_error: None,
            lang: Lang::new(detect_system_language()),
            jobs: Vec::new(),
            next_display_id: 1,
            batch_recursive: false,
            batch_filter: String::new(),
        }
    }

    pub fn rescan_ffmpeg(&mut self) {
        let (valid, path, probe, version) = detect_ffmpeg();
        self.ffmpeg_valid = valid;
        self.ffmpeg_path = path;
        self.ffprobe_path = probe;
        self.ffmpeg_version = version;
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

    pub fn enqueue_and_send(
        &mut self,
        input: PathBuf,
        job_tx: Option<&tokio::sync::mpsc::UnboundedSender<JobRequest>>,
    ) {
        let preset = match &self.selected_preset {
            Some(p) => p,
            None => {
                self.last_error = Some("No preset selected".into());
                return;
            }
        };

        let ext = preset.container.as_deref().unwrap_or("mp4");
        let output = input.with_extension(ext);

        let name = input
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let display_id = format!("gui-{}", self.next_display_id);
        self.next_display_id += 1;

        self.jobs.push(DisplayJob {
            id: display_id.clone(),
            name,
            status: DisplayJobStatus::Pending,
            progress: None,
            error: None,
        });

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

        if let Some(tx) = job_tx {
            let _ = tx.send(request);
        }
    }

    pub fn update_job_status(&mut self, _core_id: &str, status: media_core::jobs::job::JobStatus) {
        // Find the first job that matches the status transition pattern
        // Since we don't have a 1:1 mapping from core JobId to display id,
        // we match by status flow: Pending -> Running -> Completed/Failed
        use media_core::jobs::job::JobStatus;

        let display_status = match status {
            JobStatus::Pending => DisplayJobStatus::Pending,
            JobStatus::Preparing => DisplayJobStatus::Running,
            JobStatus::Running => DisplayJobStatus::Running,
            JobStatus::Completed => DisplayJobStatus::Completed,
            JobStatus::Failed(msg) => {
                // Find first pending/running job and mark it
                if let Some(job) = self.jobs.iter_mut().find(|j| {
                    j.status == DisplayJobStatus::Pending || j.status == DisplayJobStatus::Running
                }) {
                    job.status = DisplayJobStatus::Failed;
                    job.error = Some(msg);
                    return;
                }
                return;
            }
            JobStatus::Cancelled => DisplayJobStatus::Cancelled,
        };

        if let Some(job) = self.jobs.iter_mut().find(|j| {
            j.status == DisplayJobStatus::Pending && display_status == DisplayJobStatus::Running
        }) {
            job.status = display_status;
        } else if let Some(job) = self.jobs.iter_mut().find(|j| {
            j.status == DisplayJobStatus::Running && display_status == DisplayJobStatus::Completed
        }) {
            job.status = display_status;
        }
    }

    pub fn update_job_progress(&mut self, _core_id: &str, info: JobProgressInfo) {
        if let Some(job) = self
            .jobs
            .iter_mut()
            .find(|j| j.status == DisplayJobStatus::Running)
        {
            job.progress = Some(info);
        }
    }

    pub fn cancel_job(&mut self, id: &str) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            if job.status == DisplayJobStatus::Running || job.status == DisplayJobStatus::Pending {
                job.status = DisplayJobStatus::Cancelled;
            }
        }
    }

    pub fn remove_job(&mut self, id: &str) {
        self.jobs.retain(|j| j.id != id);
    }

    pub fn retry_job(&mut self, id: &str) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            if job.status == DisplayJobStatus::Failed || job.status == DisplayJobStatus::Cancelled {
                job.status = DisplayJobStatus::Pending;
                job.progress = None;
                job.error = None;
            }
        }
    }

    pub fn clear_completed_jobs(&mut self) {
        self.jobs
            .retain(|j| j.status != DisplayJobStatus::Completed);
    }

    pub fn job_display_stats(&self) -> (usize, usize, usize, usize) {
        let running = self
            .jobs
            .iter()
            .filter(|j| j.status == DisplayJobStatus::Running)
            .count();
        let pending = self
            .jobs
            .iter()
            .filter(|j| j.status == DisplayJobStatus::Pending)
            .count();
        let completed = self
            .jobs
            .iter()
            .filter(|j| j.status == DisplayJobStatus::Completed)
            .count();
        let failed = self
            .jobs
            .iter()
            .filter(|j| j.status == DisplayJobStatus::Failed)
            .count();
        (running, pending, completed, failed)
    }

    pub fn remove_file(&mut self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
        }
    }

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    pub fn set_language(&mut self, lang: Language) {
        self.lang = Lang::new(lang);
    }

    pub fn add_directory_recursive(&mut self, path: PathBuf, filter: Option<&str>) {
        if !path.is_dir() {
            return;
        }
        collect_files_recursive(&path, &mut self.files, filter);
    }

    pub fn preview_command(&self, input: &PathBuf) -> Option<String> {
        let preset = self.selected_preset.as_ref()?;
        let ext = preset.container.as_deref().unwrap_or("mp4");
        let output = input.with_extension(ext);

        let mut parts = vec!["ffmpeg".to_string()];
        parts.push("-i".to_string());
        parts.push(input.display().to_string());

        if let Some(ref v) = preset.video {
            parts.push("-c:v".to_string());
            parts.push(v.codec.clone());
            if let Some(ref q) = v.quality {
                match q {
                    media_core::pipeline::operation::QualityMode::Crf(crf) => {
                        parts.push("-crf".to_string());
                        parts.push(crf.to_string());
                    }
                    media_core::pipeline::operation::QualityMode::Bitrate(br) => {
                        parts.push("-b:v".to_string());
                        parts.push(format!("{}k", br / 1000));
                    }
                    media_core::pipeline::operation::QualityMode::Lossless => {
                        parts.push("-crf".to_string());
                        parts.push("0".to_string());
                    }
                }
            }
        }
        if let Some(ref a) = preset.audio {
            parts.push("-c:a".to_string());
            parts.push(a.codec.clone());
            if let Some(ref br) = a.bitrate {
                parts.push("-b:a".to_string());
                parts.push(br.clone());
            }
        }

        parts.push("-y".to_string());
        parts.push(output.display().to_string());

        Some(parts.join(" "))
    }
}

fn collect_files_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>, filter: Option<&str>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(pattern) = filter {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                if !matches_glob(pattern, &name) {
                    continue;
                }
            }
            if !files.contains(&path) {
                files.push(path);
            }
        } else if path.is_dir() {
            collect_files_recursive(&path, files, filter);
        }
    }
}

fn matches_glob(pattern: &str, name: &str) -> bool {
    if pattern == "*" || pattern == "*.*" {
        return true;
    }
    if let Some(ext) = pattern.strip_prefix("*.") {
        return name
            .to_lowercase()
            .ends_with(&format!(".{}", ext.to_lowercase()));
    }
    name.to_lowercase().contains(&pattern.to_lowercase())
}

fn detect_system_language() -> Language {
    for var in &["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
        if let Ok(val) = std::env::var(var) {
            if val.to_lowercase().starts_with("zh") {
                return Language::Chinese;
            }
        }
    }

    if cfg!(target_os = "windows") {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-Culture).Name"])
            .output()
        {
            let locale = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_lowercase();
            if locale.starts_with("zh") {
                return Language::Chinese;
            }
        }
    }

    Language::English
}

fn detect_ffmpeg() -> (bool, Option<String>, Option<String>, Option<String>) {
    match FfmpegLocator::new() {
        Ok(locator) => {
            let ffmpeg_path = Some(locator.ffmpeg_path().display().to_string());
            let ffprobe_path = Some(locator.ffprobe_path().display().to_string());
            let version = locator.ffmpeg_version().ok();
            (true, ffmpeg_path, ffprobe_path, version)
        }
        Err(_) => (false, None, None, None),
    }
}
