use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::MediaError;

use super::command::FfmpegCommand;
use super::progress::FfmpegProgress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Completed,
    Failed(i32),
    Killed,
}

pub struct FfmpegProcess {
    child: Option<tokio::process::Child>,
    state: ProcessState,
    stderr_lines: Vec<String>,
}

impl FfmpegProcess {
    pub async fn spawn(cmd: &FfmpegCommand) -> Result<Self, MediaError> {
        let mut command = Command::new(&cmd.program);
        command
            .args(&cmd.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        #[cfg(target_os = "windows")]
        {
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let child = command
            .spawn()
            .map_err(|e| MediaError::Other(format!("Failed to spawn ffmpeg: {}", e)))?;

        Ok(Self {
            child: Some(child),
            state: ProcessState::Running,
            stderr_lines: Vec::new(),
        })
    }

    pub async fn wait_with_progress<F>(
        &mut self,
        mut on_progress: F,
    ) -> Result<ProcessState, MediaError>
    where
        F: FnMut(&FfmpegProgress),
    {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| MediaError::Other("Process not started".into()))?;

        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        let mut progress = FfmpegProgress::default();

        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            self.stderr_lines.push(line.clone());
            progress.parse_line(&line);
            on_progress(&progress);
        }

        let status = child.wait().await.map_err(|e| {
            MediaError::Other(format!("Failed to wait for ffmpeg: {}", e))
        })?;

        if status.success() {
            self.state = ProcessState::Completed;
        } else {
            let code = status.code().unwrap_or(-1);
            self.state = ProcessState::Failed(code);
        }

        Ok(self.state.clone())
    }

    pub async fn cancel(&mut self) -> Result<(), MediaError> {
        if let Some(ref mut child) = self.child {
            child.kill().await.map_err(|e| {
                MediaError::Other(format!("Failed to kill ffmpeg: {}", e))
            })?;
            self.state = ProcessState::Killed;
        }
        Ok(())
    }

    pub fn state(&self) -> &ProcessState {
        &self.state
    }

    pub fn stderr_lines(&self) -> &[String] {
        &self.stderr_lines
    }

    pub fn stderr_tail(&self, n: usize) -> String {
        self.stderr_lines
            .iter()
            .rev()
            .take(n)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }
}
