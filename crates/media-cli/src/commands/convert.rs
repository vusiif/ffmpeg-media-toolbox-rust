use std::path::PathBuf;

use media_core::ffmpeg::command::FfmpegCommand;
use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::ffmpeg::process::FfmpegProcess;
use media_core::output::naming::NamingTemplate;
use media_core::preset::builtin::builtin_presets;
use media_core::preset::loader::PresetLoader;
use media_core::preset::preset::Preset;

use crate::args::ConvertArgs;

pub async fn run(args: ConvertArgs) -> Result<(), media_core::MediaError> {
    let locator = FfmpegLocator::new()?;

    let preset = if let Some(ref p) = args.preset {
        Some(load_preset(p)?)
    } else {
        None
    };

    let naming = NamingTemplate::default_template();

    for input in &args.inputs {
        if !input.exists() {
            eprintln!("Warning: input not found: {}", input.display());
            continue;
        }

        let ext = args
            .format
            .as_deref()
            .or_else(|| preset.as_ref().and_then(|p| p.container.as_deref()))
            .unwrap_or("mp4");

        let output_path = if let Some(ref out_dir) = args.output {
            out_dir.join(naming.render_path(&input.to_path_buf(), ext, None))
        } else {
            naming.render_path(&input.to_path_buf(), ext, None)
        };

        let mut cmd = FfmpegCommand::new(locator.ffmpeg_path().clone());
        cmd.arg("-i").arg(input);

        if let Some(ref p) = preset {
            if let Some(ref v) = p.video {
                cmd.arg("-c:v").arg(&v.codec);
                if let Some(ref q) = v.quality {
                    match q {
                        media_core::pipeline::operation::QualityMode::Crf(crf) => {
                            cmd.arg("-crf").arg(crf.to_string());
                        }
                        media_core::pipeline::operation::QualityMode::Bitrate(br) => {
                            cmd.arg("-b:v").arg(format!("{}k", br / 1000));
                        }
                        media_core::pipeline::operation::QualityMode::Lossless => {
                            cmd.arg("-crf").arg("0");
                        }
                    }
                }
            }
            if let Some(ref a) = p.audio {
                cmd.arg("-c:a").arg(&a.codec);
                if let Some(ref br) = a.bitrate {
                    cmd.arg("-b:a").arg(br);
                }
            }
        } else if let Some(ref codec) = args.codec {
            cmd.arg("-c:v").arg(codec);
        }

        cmd.arg("-y").arg(&output_path);

        if args.dry_run {
            println!("{}", cmd.to_command_string());
            continue;
        }

        if !args.quiet {
            println!("{} -> {}", input.display(), output_path.display());
        }

        let mut process = FfmpegProcess::spawn(&cmd).await?;
        let state = process.wait_with_progress(|_p| {}).await?;

        match state {
            media_core::ffmpeg::process::ProcessState::Completed => {
                if !args.quiet {
                    println!("  Done");
                }
            }
            media_core::ffmpeg::process::ProcessState::Failed(code) => {
                eprintln!("  Failed (exit code {})", code);
                let tail = process.stderr_tail(5);
                if !tail.is_empty() {
                    eprintln!("  {}", tail.replace('\n', "\n  "));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn load_preset(name_or_path: &str) -> Result<Preset, media_core::MediaError> {
    for bp in builtin_presets() {
        if bp.name.eq_ignore_ascii_case(name_or_path) {
            return Ok(bp);
        }
    }

    let path = PathBuf::from(name_or_path);
    if path.exists() {
        let loader = PresetLoader::new();
        return loader.load_from_file(&path);
    }

    Err(media_core::MediaError::InvalidPreset(format!(
        "Preset not found: {}",
        name_or_path
    )))
}
