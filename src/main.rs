mod api;
mod cli;
mod configuration;
mod database;
mod errors;
mod extension;
mod flags;
mod operation;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use configuration::profile::Profile;
use errors::Result;
use flags::flags::Flags;
use operation::configurable::Configurable;
use operation::runnable::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let profile = Profile::try_from(&cli)?;
	let flags = Flags::try_configure_from(cli, profile).await?;
	flags.try_run().await?;

	Ok(())
}
