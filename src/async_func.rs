use crate::install;
use crate::query;
use std::path::PathBuf;

pub fn create_install_tasks(extensions: &[String], download_path: &PathBuf) -> Vec<tokio::task::JoinHandle<()>> {
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

pub async fn install_extension_task(ext: &String, download_path: &PathBuf) {
	match query::query_extension(&ext).await {
		Ok(e) => {
			println!("Installing extension {}.", ext);
			match install::install_extension(&e, download_path).await {
				Ok(_) => println!("Successfully installed extension {}.", &ext),
				Err(error) => eprintln!("Error installing extension {}. Error: {}", ext, error),
			};
		}

		Err(error) => {
			eprintln!("Error: {}", error);
		}
	}
}

pub async fn execute_tasks(tasks: Vec<tokio::task::JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}
