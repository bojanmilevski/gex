mod addon;
mod api;
mod cli;
mod configuration;
mod errors;
mod extension;
mod flags;
mod manifest;
mod operation;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use errors::Result;
use flags::flags::Flags;
use operation::configurable::Configurable;
use operation::runnable::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let flags = Flags::try_configure_from(cli).await?;
	flags.try_run().await?;

	Ok(())
}
