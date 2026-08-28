use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "media-rs", version, about = "Fast media processor powered by FFmpeg")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Probe media file for information
    Probe(ProbeArgs),

    /// Convert video/audio files
    Convert(ConvertArgs),

    /// Image operations (crop, resize, rotate, flip, join)
    Image(ImageArgs),

    /// Batch process files from directory
    Batch(BatchArgs),

    /// Manage presets
    Preset(PresetArgs),
}

#[derive(Parser)]
pub struct ProbeArgs {
    /// Input file path
    pub input: PathBuf,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser)]
pub struct ConvertArgs {
    /// Input file(s)
    pub inputs: Vec<PathBuf>,

    /// Output format
    #[arg(short, long)]
    pub format: Option<String>,

    /// Video codec
    #[arg(long)]
    pub codec: Option<String>,

    /// Preset name or path
    #[arg(short, long)]
    pub preset: Option<String>,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Show command without executing
    #[arg(long)]
    pub dry_run: bool,

    /// Quiet mode
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Parser)]
pub struct ImageArgs {
    /// Input file(s)
    pub inputs: Vec<PathBuf>,

    /// Resize width
    #[arg(long)]
    pub resize: Option<String>,

    /// Output format
    #[arg(short, long)]
    pub format: Option<String>,

    /// Preset name or path
    #[arg(short, long)]
    pub preset: Option<String>,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Show command without executing
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Parser)]
pub struct BatchArgs {
    /// Input directory
    pub input: PathBuf,

    /// Process subdirectories recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Preset name or path
    #[arg(short, long)]
    pub preset: String,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Show commands without executing
    #[arg(long)]
    pub dry_run: bool,

    /// Quiet mode
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Parser)]
pub struct PresetArgs {
    #[command(subcommand)]
    pub action: PresetAction,
}

#[derive(Subcommand)]
pub enum PresetAction {
    /// List available presets
    List,

    /// Show preset details
    Show {
        /// Preset name
        name: String,
    },
}
