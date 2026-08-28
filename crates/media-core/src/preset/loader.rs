use std::path::{Path, PathBuf};

use crate::MediaError;

use super::preset::Preset;

pub struct PresetLoader {
    search_dirs: Vec<PathBuf>,
}

impl Default for PresetLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetLoader {
    pub fn new() -> Self {
        Self {
            search_dirs: Vec::new(),
        }
    }

    pub fn add_dir(&mut self, dir: PathBuf) {
        self.search_dirs.push(dir);
    }

    pub fn load_from_file(&self, path: &Path) -> Result<Preset, MediaError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            MediaError::InvalidPreset(format!("Failed to read preset {}: {}", path.display(), e))
        })?;
        let preset: Preset = serde_json::from_str(&content)?;
        Ok(preset)
    }

    pub fn load_all(&self) -> Vec<Result<Preset, MediaError>> {
        let mut results = Vec::new();

        for dir in &self.search_dirs {
            if !dir.exists() {
                continue;
            }

            let entries = match std::fs::read_dir(dir) {
                Ok(e) => e,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "json") {
                    results.push(self.load_from_file(&path));
                }
            }
        }

        results
    }
}
