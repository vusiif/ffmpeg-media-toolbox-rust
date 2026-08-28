use std::ffi::OsString;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfmpegCommand {
    pub program: PathBuf,
    pub args: Vec<OsString>,
}

impl FfmpegCommand {
    pub fn new(program: PathBuf) -> Self {
        Self {
            program,
            args: Vec::new(),
        }
    }

    pub fn arg<S: Into<OsString>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    pub fn to_command_string(&self) -> String {
        let mut parts = vec![self.program.display().to_string()];
        for arg in &self.args {
            let s = arg.to_string_lossy().to_string();
            if s.contains(' ') {
                parts.push(format!("\"{}\"", s));
            } else {
                parts.push(s);
            }
        }
        parts.join(" ")
    }
}
