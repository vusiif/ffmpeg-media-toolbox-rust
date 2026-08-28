use super::preset::{AudioPreset, ImagePreset, Preset, PresetType, VideoPreset};
use crate::pipeline::operation::QualityMode;

pub fn builtin_presets() -> Vec<Preset> {
    vec![
        h264_compatible(),
        h265_high_quality(),
        h265_small(),
        mp3_320k(),
        aac_256k(),
        opus_160k(),
        flac(),
        webp_high_quality(),
        webp_small(),
        jpeg_high_quality(),
        png_lossless(),
    ]
}

fn h264_compatible() -> Preset {
    Preset {
        name: "H264 Compatible".into(),
        preset_type: PresetType::Video,
        description: "H.264 + AAC, wide compatibility".into(),
        video: Some(VideoPreset {
            codec: "libx264".into(),
            quality: Some(QualityMode::Crf(23)),
            fps: None,
            extra_args: vec![],
        }),
        audio: Some(AudioPreset {
            codec: "aac".into(),
            bitrate: Some("192k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("mp4".into()),
        builtin: true,
    }
}

fn h265_high_quality() -> Preset {
    Preset {
        name: "H265 High Quality".into(),
        preset_type: PresetType::Video,
        description: "H.265 high quality, good for archiving".into(),
        video: Some(VideoPreset {
            codec: "libx265".into(),
            quality: Some(QualityMode::Crf(20)),
            fps: None,
            extra_args: vec![],
        }),
        audio: Some(AudioPreset {
            codec: "aac".into(),
            bitrate: Some("256k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("mp4".into()),
        builtin: true,
    }
}

fn h265_small() -> Preset {
    Preset {
        name: "H265 Small".into(),
        preset_type: PresetType::Video,
        description: "H.265 small file size, good for sharing".into(),
        video: Some(VideoPreset {
            codec: "libx265".into(),
            quality: Some(QualityMode::Crf(28)),
            fps: None,
            extra_args: vec![],
        }),
        audio: Some(AudioPreset {
            codec: "aac".into(),
            bitrate: Some("128k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("mp4".into()),
        builtin: true,
    }
}

fn mp3_320k() -> Preset {
    Preset {
        name: "MP3 320k".into(),
        preset_type: PresetType::Audio,
        description: "MP3 320kbps high quality".into(),
        video: None,
        audio: Some(AudioPreset {
            codec: "libmp3lame".into(),
            bitrate: Some("320k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("mp3".into()),
        builtin: true,
    }
}

fn aac_256k() -> Preset {
    Preset {
        name: "AAC 256k".into(),
        preset_type: PresetType::Audio,
        description: "AAC 256kbps".into(),
        video: None,
        audio: Some(AudioPreset {
            codec: "aac".into(),
            bitrate: Some("256k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("m4a".into()),
        builtin: true,
    }
}

fn opus_160k() -> Preset {
    Preset {
        name: "Opus 160k".into(),
        preset_type: PresetType::Audio,
        description: "Opus 160kbps, excellent quality per byte".into(),
        video: None,
        audio: Some(AudioPreset {
            codec: "libopus".into(),
            bitrate: Some("160k".into()),
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("ogg".into()),
        builtin: true,
    }
}

fn flac() -> Preset {
    Preset {
        name: "FLAC".into(),
        preset_type: PresetType::Audio,
        description: "FLAC lossless audio".into(),
        video: None,
        audio: Some(AudioPreset {
            codec: "flac".into(),
            bitrate: None,
            sample_rate: None,
            channels: None,
        }),
        image: None,
        container: Some("flac".into()),
        builtin: true,
    }
}

fn webp_high_quality() -> Preset {
    Preset {
        name: "WebP High Quality".into(),
        preset_type: PresetType::Image,
        description: "WebP high quality images".into(),
        video: None,
        audio: None,
        image: Some(ImagePreset {
            format: "webp".into(),
            quality: Some(90),
            max_width: None,
            max_height: None,
        }),
        container: None,
        builtin: true,
    }
}

fn webp_small() -> Preset {
    Preset {
        name: "WebP Small".into(),
        preset_type: PresetType::Image,
        description: "WebP small file size".into(),
        video: None,
        audio: None,
        image: Some(ImagePreset {
            format: "webp".into(),
            quality: Some(75),
            max_width: Some(1920),
            max_height: Some(1080),
        }),
        container: None,
        builtin: true,
    }
}

fn jpeg_high_quality() -> Preset {
    Preset {
        name: "JPEG High Quality".into(),
        preset_type: PresetType::Image,
        description: "JPEG high quality".into(),
        video: None,
        audio: None,
        image: Some(ImagePreset {
            format: "jpeg".into(),
            quality: Some(95),
            max_width: None,
            max_height: None,
        }),
        container: None,
        builtin: true,
    }
}

fn png_lossless() -> Preset {
    Preset {
        name: "PNG Lossless".into(),
        preset_type: PresetType::Image,
        description: "PNG lossless".into(),
        video: None,
        audio: None,
        image: Some(ImagePreset {
            format: "png".into(),
            quality: None,
            max_width: None,
            max_height: None,
        }),
        container: None,
        builtin: true,
    }
}
