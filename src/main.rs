mod addon;
mod api;
mod cli;
mod core;
mod database;
mod errors;
mod extension;
mod flags;
mod install;
mod manifest;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use errors::Result;
use flags::configurable::Configurable;
use flags::flags::Flags;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let flags = Flags::try_configure_from(&cli).await?;

	if cli.operation.search.is_none() {
		let tasks = core::create_install_tasks(&flags);
		core::execute_tasks(tasks).await;
	} else {
		for extension in &flags.search.extensions {
			println!("{}", extension);
		}
	}

	database::add_extension(&flags).await?;

	Ok(())
}
