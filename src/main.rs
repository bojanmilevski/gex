mod args;
mod core;
mod errors;
mod extension;
mod flags;
mod install;
mod query;

use crate::errors::Result;
use args::Args;
use clap::Parser;
use flags::Configurable;
use flags::Flags;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let flags = Flags::configure_from(&args).await?;

	if args.search.is_empty() {
		let install_tasks = core::create_install_tasks(&flags);
		core::execute_tasks(install_tasks).await;
	} else {
		for extension in flags.search.extensions {
			println!("{}", extension);
		}
	}

	Ok(())
}
