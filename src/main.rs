mod args;
mod core;
mod errors;
mod extension;
mod flags;
mod install;
mod query;

use args::Args;
use flags::Configurable;
use flags::Flags;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let flags = Flags::configure_from(&args).await?;

	let install_tasks = core::create_install_tasks(&flags.extensions, &flags.profile);
	core::execute_tasks(install_tasks).await;

	Ok(())
}
