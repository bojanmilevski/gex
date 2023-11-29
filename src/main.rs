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
use colored::Colorize;
use itertools::Itertools;
use profile::Profile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	if !args.query.is_empty() {
		let extensions = query::query_extensions_list(args.query).await?;
		let len = extensions.len();
		extensions.iter().sorted().for_each(|e| println!("{}\n", e));
		println!("{}: {}", "Total extensions queried".bold().bright_blue(), len);
		return Ok(());
	}

	let profile = Profile::from(&args.profile, &args.browser).await?;

	let install_tasks = async_func::create_install_tasks(&args.install, &profile);
	async_func::execute_tasks(install_tasks).await;

	Ok(())
}
