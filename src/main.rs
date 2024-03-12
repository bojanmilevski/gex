mod addons_json_database;
mod api;
mod cli;
mod configuration;
mod core;
mod errors;
mod extension;
mod extensions_json_database;
mod manifest;
mod operation;
mod progress_bar;
mod traits;

use clap::Parser;
use cli::Cli;
use core::Core;
use errors::Result;
use traits::configurable::Configurable;
use traits::runnable::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let core = Core::try_configure_from(cli).await?;
	core.try_run().await?;

	Ok(())
}
