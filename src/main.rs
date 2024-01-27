mod api;
mod cli;
mod database;
mod errors;
mod extension;
mod flags;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use errors::Result;
use flags::configurable::Configurable;
use flags::flags::Flags;
use flags::runnable::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let flags = Flags::try_configure_from(&cli).await?;
	flags.try_run(&flags).await?;

	Ok(())
}
