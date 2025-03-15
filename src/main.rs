#[macro_use]
extern crate tracing;

use clap::Parser;
use std::path::PathBuf;
use vexelstrom::{config::Configuration, consts::VERSION, http::state::AppState};

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
    let config = Configuration::load(args.config).await?;

    let db_pool = vexelstrom::db::connect(&config).await?;

    info!(version = VERSION, "starting vexelstrom");

    let state = AppState {
        pool: db_pool,
        unit: (),
    };
    let handle = tokio::spawn(vexelstrom::http::serve(config, state));

    tokio::select! {
        result = handle => result??,
    }

    Ok(())
}
