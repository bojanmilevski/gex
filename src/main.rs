mod api;
mod cli;
mod configurable;
mod configuration;
mod core;
mod database;
mod errors;
mod extension;
mod manifest;
mod operation;
mod progress_bar;
mod runnable;

use clap::Parser;
use cli::Cli;
use configurable::Configurable;
use core::Core;
use errors::Result;
use runnable::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let core = Core::try_configure_from(cli).await?;
	core.try_run().await?;

	Ok(())
}
