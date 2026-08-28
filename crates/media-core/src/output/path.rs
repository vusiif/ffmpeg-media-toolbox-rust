use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::MediaError;

use super::naming::NamingTemplate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictPolicy {
    Ask,
    Replace,
    Rename,
    Skip,
}

impl ConflictPolicy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ask => "Ask",
            Self::Replace => "Replace",
            Self::Rename => "Rename",
            Self::Skip => "Skip",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputPath {
    pub directory: Option<PathBuf>,
    pub naming: NamingTemplate,
    pub conflict: ConflictPolicy,
    pub mirror_structure: bool,
}

impl Default for OutputPath {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputPath {
    pub fn new() -> Self {
        Self {
            directory: None,
            naming: NamingTemplate::default_template(),
            conflict: ConflictPolicy::Rename,
            mirror_structure: false,
        }
    }

    pub fn resolve(&self, input: &Path, new_ext: &str, index: Option<usize>) -> PathBuf {
        let output_dir = self
            .directory
            .clone()
            .unwrap_or_else(|| input.parent().unwrap_or(Path::new(".")).to_path_buf());

        let filename = self.naming.render_path(input, new_ext, index);

        let file_name = filename
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        output_dir.join(file_name)
    }

    pub fn resolve_with_conflict(
        &self,
        input: &Path,
        new_ext: &str,
        index: Option<usize>,
    ) -> Result<PathBuf, MediaError> {
        let mut output = self.resolve(input, new_ext, index);

        if !output.exists() {
            return Ok(output);
        }

        match self.conflict {
            ConflictPolicy::Replace => Ok(output),
            ConflictPolicy::Skip => Err(MediaError::OutputPath("Skipped: file exists".into())),
            ConflictPolicy::Rename => {
                let stem = output
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                let ext = output
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default();

                for i in 1..10000 {
                    let new_name = format!("{}_{}", stem, i);
                    output.set_file_name(&new_name);
                    if !ext.is_empty() {
                        output.set_extension(&ext);
                    }
                    if !output.exists() {
                        return Ok(output);
                    }
                }

                Err(MediaError::OutputPath(
                    "Could not find unused filename".into(),
                ))
            }
            ConflictPolicy::Ask => Err(MediaError::OutputPath(
                "File exists, user decision needed".into(),
            )),
        }
    }
}
