mod addon;
mod cli;
mod database;
mod operation;
mod profile;
mod progress_bar;
mod traits;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use operation::operation::Operation;
use traits::Runnable;

async fn run() -> Result<()> {
	let cli = Cli::parse();
	let mut operation = Operation::try_init(cli.operation).await?;
	operation.try_run().await?;

	Ok(())
}

#[tokio::main]
async fn main() {
	if let Err(err) = run().await {
		eprintln!("{}: {err}", "Error".bold().red());
	}
}
