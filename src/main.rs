mod args;
mod core;
mod errors;
mod extension;
mod flags;
mod install;
mod query;

use args::Args;
use errors::FlagsError;
use flags::Configurable;
use flags::Flags;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), FlagsError> {
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
