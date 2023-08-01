mod args;
mod config;
mod errors;
mod install;
mod profile;
mod query;

use args::Args;
use errors::ArgsError;
use install::install_extensions;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ArgsError> {
	let args = Args::parse().validate_args().await?;
	let download_path = args.get_download_path().await?;

	if !args.install.is_empty() {
		match install_extensions(&args.install, &download_path).await {
			Ok(_) => println!("Successfully installed extension."),
			Err(err) => eprintln!("Error installing extension: {err}"),
		}
	}

	Ok(())
}
