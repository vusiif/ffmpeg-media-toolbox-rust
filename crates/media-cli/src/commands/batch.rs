use std::path::PathBuf;

use media_core::ffmpeg::command::FfmpegCommand;
use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::ffmpeg::process::FfmpegProcess;
use media_core::output::naming::NamingTemplate;
use media_core::preset::builtin::builtin_presets;
use media_core::preset::loader::PresetLoader;
use media_core::preset::preset::Preset;

use crate::args::BatchArgs;

pub async fn run(args: BatchArgs) -> Result<(), media_core::MediaError> {
    let locator = FfmpegLocator::new()?;
    let preset = load_preset(&args.preset)?;
    let naming = NamingTemplate::default_template();

    let files = collect_files(&args.input, args.recursive)?;

    if files.is_empty() {
        println!("No files found in {}", args.input.display());
        return Ok(());
    }

    println!("Processing {} files", files.len());

    let ext = preset.container.as_deref().unwrap_or("mp4");

    for (i, input) in files.iter().enumerate() {
        let output_path = if let Some(ref out_dir) = args.output {
            out_dir.join(naming.render_path(&input.to_path_buf(), ext, Some(i)))
        } else {
            naming.render_path(&input.to_path_buf(), ext, Some(i))
        };

        let mut cmd = FfmpegCommand::new(locator.ffmpeg_path().clone());
        cmd.arg("-i").arg(input);

        if let Some(ref v) = preset.video {
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
        if let Some(ref a) = preset.audio {
            cmd.arg("-c:a").arg(&a.codec);
            if let Some(ref br) = a.bitrate {
                cmd.arg("-b:a").arg(br);
            }
        }

        cmd.arg("-y").arg(&output_path);

        if args.dry_run {
            println!("{}", cmd.to_command_string());
            continue;
        }

        if !args.quiet {
            println!("[{}/{}] {} -> {}", i + 1, files.len(), input.display(), output_path.display());
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
            }
            _ => {}
        }
    }

    Ok(())
}

fn collect_files(dir: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>, media_core::MediaError> {
    let mut files = Vec::new();
    collect_files_inner(dir, recursive, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(
    dir: &PathBuf,
    recursive: bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), media_core::MediaError> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| media_core::MediaError::Io(e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() && recursive {
            collect_files_inner(&path, recursive, files)?;
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
