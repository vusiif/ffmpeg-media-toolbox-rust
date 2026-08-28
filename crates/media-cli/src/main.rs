mod args;
mod commands;

use clap::Parser;

use args::{Cli, Commands};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Probe(cmd) => commands::probe::run(cmd).await,
        Commands::Convert(cmd) => commands::convert::run(cmd).await,
        Commands::Image(cmd) => commands::image::run(cmd).await,
        Commands::Batch(cmd) => commands::batch::run(cmd).await,
        Commands::Preset(cmd) => commands::preset::run(cmd).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
