mod addon;
mod api;
mod cli;
mod configuration;
mod core;
mod database;
mod operation;
mod progress_bar;
mod traits;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use core::Core;
use traits::configurable::Configurable;
use traits::runnable::Runnable;

async fn run() -> Result<()> {
	let cli = Cli::parse();
	let mut core = Core::try_configure_from(cli).await?;
	core.try_run().await?;

	Ok(())
}

#[tokio::main]
async fn main() {
	if let Err(err) = run().await {
		eprintln!("{}: {err}", "Error".bold().red());
	}
}
