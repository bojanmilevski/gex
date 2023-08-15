mod args;
mod config;
mod errors;
mod install;
mod profile;
mod query;

use args::Args;
use clap::Parser;
use errors::ArgsError;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), ArgsError> {
	let args = Args::parse().validate_args().await?;
	let download_path = args.get_download_path().await?;
	let mut tasks = Vec::new();

	for ext in args.install {
		let ext_clone = ext.clone();
		let dp_clone = download_path.clone();

		let task = task::spawn(async move {
			let query_result = query::query_extension(&ext_clone);
			eprintln!("Installing extension {}.", &ext_clone);

			if let Some(extension) = query_result.await.unwrap().results.first() {
				install::install_extension(extension, &dp_clone).await.unwrap();
				eprintln!("Successfully installed extension {}.", &ext_clone);
			} else {
				eprintln!("Extension {} not found.", &ext_clone);
			}
		});

		tasks.push(task);
	}

	for task in tasks {
		let _ = task.await;
	}

	Ok(())
}
