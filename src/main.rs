mod args;
mod async_func;
mod browser;
mod config;
mod errors;
mod extension;
mod install;
mod profile;
mod query;

use args::Args;
use clap::Parser;
use errors::ProfileError;
use profile::Profile;

#[tokio::main]
async fn main() -> Result<(), ProfileError> {
	let args = Args::parse();
	let profile = Profile::from(&args.profile, &args.browser.path).await?;

	let install_tasks = async_func::create_install_tasks(&args.install, &profile.path);
	async_func::execute_tasks(install_tasks).await;

	Ok(())
}
