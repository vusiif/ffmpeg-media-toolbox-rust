use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropParams {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AspectRatio {
    Free,
    Square,
    R4_3,
    R3_2,
    R16_9,
    R9_16,
}

impl AspectRatio {
    pub fn to_ratio(self) -> Option<(f64, f64)> {
        match self {
            Self::Free => None,
            Self::Square => Some((1.0, 1.0)),
            Self::R4_3 => Some((4.0, 3.0)),
            Self::R3_2 => Some((3.0, 2.0)),
            Self::R16_9 => Some((16.0, 9.0)),
            Self::R9_16 => Some((9.0, 16.0)),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Free => "Free",
            Self::Square => "1:1",
            Self::R4_3 => "4:3",
            Self::R3_2 => "3:2",
            Self::R16_9 => "16:9",
            Self::R9_16 => "9:16",
        }
    }
}

impl AspectRatio {
    pub fn all() -> &'static [AspectRatio] {
        &[
            Self::Free,
            Self::Square,
            Self::R4_3,
            Self::R3_2,
            Self::R16_9,
            Self::R9_16,
        ]
    }
}
