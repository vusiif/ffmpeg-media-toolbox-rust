use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::ffmpeg::probe;

use crate::args::ProbeArgs;

pub async fn run(args: ProbeArgs) -> Result<(), media_core::MediaError> {
    let locator = FfmpegLocator::new()?;
    let media_file = probe::probe_file(&locator, &args.input)?;

    if args.json {
        let json = serde_json::json!({
            "path": media_file.path.display().to_string(),
            "file_size": media_file.file_size,
            "duration_secs": media_file.duration.map(|d| d.as_secs_f64()),
            "format": media_file.format_name,
            "video_streams": media_file.video_streams.iter().map(|s| {
                serde_json::json!({
                    "index": s.index,
                    "codec": s.codec,
                    "width": s.width,
                    "height": s.height,
                    "fps": s.fps,
                })
            }).collect::<Vec<_>>(),
            "audio_streams": media_file.audio_streams.iter().map(|s| {
                serde_json::json!({
                    "index": s.index,
                    "codec": s.codec,
                    "sample_rate": s.sample_rate,
                    "channels": s.channels,
                })
            }).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("File: {}", media_file.path.display());
        println!("Format: {}", media_file.format_name);
        println!("Size: {}", format_size(media_file.file_size));
        if let Some(d) = media_file.duration {
            println!("Duration: {:.2}s", d.as_secs_f64());
        }
        for vs in &media_file.video_streams {
            println!(
                "Video #{}: {} {}x{} {:.2}fps",
                vs.index,
                vs.codec,
                vs.width,
                vs.height,
                vs.fps.unwrap_or(0.0)
            );
        }
        for as_ in &media_file.audio_streams {
            println!(
                "Audio #{}: {} {}Hz {}ch",
                as_.index, as_.codec, as_.sample_rate, as_.channels
            );
        }
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
