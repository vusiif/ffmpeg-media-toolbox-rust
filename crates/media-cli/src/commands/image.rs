use media_core::ffmpeg::command::FfmpegCommand;
use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::ffmpeg::process::FfmpegProcess;
use media_core::output::naming::NamingTemplate;
use media_core::pipeline::compiler::PipelineCompiler;
use media_core::pipeline::operation::ImageOperation;

use crate::args::ImageArgs;

pub async fn run(args: ImageArgs) -> Result<(), media_core::MediaError> {
    let locator = FfmpegLocator::new()?;
    let naming = NamingTemplate::default_template();

    let ext = args.format.as_deref().unwrap_or("png");

    let mut operations = Vec::new();

    if let Some(ref resize_str) = args.resize {
        let parts: Vec<&str> = resize_str.split('x').collect();
        let width = parts.first().and_then(|s| s.parse().ok());
        let height = parts.get(1).and_then(|s| s.parse().ok());

        operations.push(ImageOperation::Resize(
            media_core::pipeline::operation::ResizeOperation {
                width,
                height,
                mode: media_core::pipeline::operation::ResizeMode::Fit,
                keep_aspect: true,
                prevent_upscale: false,
            },
        ));
    }

    let fg = PipelineCompiler::compile_image_operations(&operations)?;

    for input in &args.inputs {
        if !input.exists() {
            eprintln!("Warning: input not found: {}", input.display());
            continue;
        }

        let output_path = if let Some(ref out_dir) = args.output {
            out_dir.join(naming.render_path(&input.to_path_buf(), ext, None))
        } else {
            naming.render_path(&input.to_path_buf(), ext, None)
        };

        let mut cmd = FfmpegCommand::new(locator.ffmpeg_path().clone());
        cmd.arg("-i").arg(input);

        if !fg.is_empty() {
            let filter_args = fg.to_filter_complex();
            cmd.args(filter_args);
        }

        cmd.arg("-y").arg(&output_path);

        if args.dry_run {
            println!("{}", cmd.to_command_string());
            continue;
        }

        println!("{} -> {}", input.display(), output_path.display());

        let mut process = FfmpegProcess::spawn(&cmd).await?;
        let state = process.wait_with_progress(|_p| {}).await?;

        match state {
            media_core::ffmpeg::process::ProcessState::Completed => {
                println!("  Done");
            }
            media_core::ffmpeg::process::ProcessState::Failed(code) => {
                eprintln!("  Failed (exit code {})", code);
            }
            _ => {}
        }
    }

    Ok(())
}
