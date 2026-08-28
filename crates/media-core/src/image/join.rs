use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinParams {
    pub direction: JoinDirection,
    pub spacing: u32,
    pub margin: u32,
    pub background: String,
    pub cell_size: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JoinDirection {
    Horizontal,
    Vertical,
    Grid,
}

impl JoinDirection {
    pub fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "Horizontal",
            Self::Vertical => "Vertical",
            Self::Grid => "Grid",
        }
    }

    pub fn all() -> &'static [JoinDirection] {
        &[Self::Horizontal, Self::Vertical, Self::Grid]
    }
}

impl Default for JoinParams {
    fn default() -> Self {
        Self {
            direction: JoinDirection::Horizontal,
            spacing: 0,
            margin: 0,
            background: "white".to_string(),
            cell_size: None,
        }
    }
}
