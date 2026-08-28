use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RotateAngle {
    CW90,
    CCW90,
    R180,
}

impl RotateAngle {
    pub fn label(self) -> &'static str {
        match self {
            Self::CW90 => "90° CW",
            Self::CCW90 => "90° CCW",
            Self::R180 => "180°",
        }
    }

    pub fn all() -> &'static [RotateAngle] {
        &[Self::CW90, Self::CCW90, Self::R180]
    }
}
