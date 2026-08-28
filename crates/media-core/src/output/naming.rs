use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NamingTemplate {
    template: String,
}

impl NamingTemplate {
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    pub fn default_template() -> Self {
        Self::new("{name}")
    }

    pub fn render(
        &self,
        name: &str,
        ext: &str,
        index: Option<usize>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> String {
        let mut result = self.template.clone();
        result = result.replace("{name}", name);
        result = result.replace("{ext}", ext);

        if let Some(i) = index {
            result = result.replace("{index}", &i.to_string());
        }
        if let Some(w) = width {
            result = result.replace("{width}", &w.to_string());
        }
        if let Some(h) = height {
            result = result.replace("{height}", &h.to_string());
        }

        result
    }

    pub fn render_path(
        &self,
        source: &std::path::Path,
        new_ext: &str,
        index: Option<usize>,
    ) -> PathBuf {
        let stem = source
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let ext = new_ext.trim_start_matches('.');
        let rendered = self.render(&stem, ext, index, None, None);

        let mut output = source.to_path_buf();
        output.set_file_name(rendered);
        if !ext.is_empty() {
            output.set_extension(ext);
        }
        output
    }
}
