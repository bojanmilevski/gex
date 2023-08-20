mod args;
mod config;
mod errors;
mod install;
mod profile;
mod query;

use errors::ProfileError;

use args::Args;
use clap::Parser;

fn create_install_tasks(extensions: &[String], download_path: &String) -> Vec<tokio::task::JoinHandle<()>> {
	extensions
		.iter()
		.map(|ext| {
			let ext_clone = ext.clone();
			let dp_clone = download_path.to_owned();
			tokio::task::spawn(async move {
				install_extension_task(&ext_clone, &dp_clone).await;
			})
		})
		.collect()
}

async fn install_extension_task(ext: &String, download_path: &String) {
	match query::query_extension(&ext).await {
		Ok(ext_info) => {
			println!("Installing extension {}.", ext);
			match install::install_extension(&ext_info, download_path).await {
				Ok(_) => println!("Successfully installed extension {}.", &ext),
				Err(error) => eprintln!("Error installing extension {}. Error: {}", ext, error),
			};
		}

		Err(error) => {
			eprintln!("{}", error);
		}
	}
}

async fn execute_tasks(tasks: Vec<tokio::task::JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}

#[tokio::main]
async fn main() -> Result<(), ProfileError> {
	let args = Args::parse();
	let download_path = args.get_download_path().await?;
	let install_tasks = create_install_tasks(&args.install, &download_path);
	execute_tasks(install_tasks).await;

	Ok(())
}
