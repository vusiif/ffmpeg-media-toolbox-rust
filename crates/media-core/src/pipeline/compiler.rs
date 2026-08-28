use crate::MediaError;

use super::filtergraph::FilterGraph;
use super::operation::{
    CropOperation, FlipOperation, ImageOperation, ResizeMode, ResizeOperation, RotateOperation,
};

pub struct PipelineCompiler;

impl PipelineCompiler {
    pub fn compile_image_operations(ops: &[ImageOperation]) -> Result<FilterGraph, MediaError> {
        let mut fg = FilterGraph::new();

        for op in ops {
            match op {
                ImageOperation::Crop(crop) => {
                    fg.push(Self::compile_crop(crop));
                }
                ImageOperation::Resize(resize) => {
                    fg.push(Self::compile_resize(resize)?);
                }
                ImageOperation::Rotate(rotate) => {
                    fg.push(Self::compile_rotate(*rotate));
                }
                ImageOperation::Flip(flip) => {
                    fg.push(Self::compile_flip(*flip));
                }
            }
        }

        Ok(fg)
    }

    fn compile_crop(crop: &CropOperation) -> String {
        format!("crop={}:{}:{}:{}", crop.width, crop.height, crop.x, crop.y)
    }

    fn compile_resize(r: &ResizeOperation) -> Result<String, MediaError> {
        let w = r.width.unwrap_or(0);
        let h = r.height.unwrap_or(0);

        if w == 0 && h == 0 {
            return Err(MediaError::InvalidOperation(
                "Resize requires at least one dimension".into(),
            ));
        }

        let scale = match r.mode {
            ResizeMode::Exact => {
                if r.keep_aspect {
                    format!("scale={}:{}", w, h)
                } else {
                    format!("scale={}:{}", w, h)
                }
            }
            ResizeMode::Fit => {
                if r.prevent_upscale {
                    format!("scale='min({},{})':min({},{})", w, w, h, h)
                } else if r.keep_aspect {
                    format!("scale={}:{}", w, h)
                } else {
                    format!("scale={}:{}", w, h)
                }
            }
            ResizeMode::Fill => format!("scale={}:{}:force_original_aspect_ratio=increase", w, h),
            ResizeMode::Percentage(pct) => {
                let factor = pct / 100.0;
                format!("scale='trunc(iw*{0}/2)*2:trunc(ih*{0}/2)*2'", factor)
            }
        };

        Ok(scale)
    }

    fn compile_rotate(rotate: RotateOperation) -> String {
        match rotate {
            RotateOperation::CW90 => "transpose=1".to_string(),
            RotateOperation::CCW90 => "transpose=2".to_string(),
            RotateOperation::R180 => "transpose=1,transpose=1".to_string(),
        }
    }

    fn compile_flip(flip: FlipOperation) -> String {
        match flip {
            FlipOperation::Horizontal => "hflip".to_string(),
            FlipOperation::Vertical => "vflip".to_string(),
        }
    }
}
