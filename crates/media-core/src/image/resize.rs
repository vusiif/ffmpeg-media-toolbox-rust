use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub mode: ResizeMode,
    pub keep_aspect: bool,
    pub prevent_upscale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResizeMode {
    Exact,
    Fit,
    Fill,
    Percentage(f64),
}

impl ResizeMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Fit => "Fit",
            Self::Fill => "Fill",
            Self::Percentage(_) => "Percentage",
        }
    }
}
