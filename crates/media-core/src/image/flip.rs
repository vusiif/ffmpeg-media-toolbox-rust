use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

impl FlipDirection {
    pub fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "Horizontal",
            Self::Vertical => "Vertical",
        }
    }

    pub fn all() -> &'static [FlipDirection] {
        &[Self::Horizontal, Self::Vertical]
    }
}
