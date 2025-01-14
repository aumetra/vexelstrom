#[macro_use]
extern crate tracing;

use clap::Parser;
use std::path::PathBuf;
use vexelstrom::{config::Configuration, consts::VERSION};

#[derive(Parser)]
#[clap(about, version = VERSION)]
struct Cli {
    /// Path to the configuration
    #[arg(short, long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let args = Cli::parse();
    let _config = Configuration::load(args.config).await?;

    info!(version = VERSION, "starting vexelstrom");

    Ok(())
}
