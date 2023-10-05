use crate::install;
use crate::profile::Profile;
use crate::query;
use tokio::task::JoinHandle;

pub fn create_install_tasks(extensions: &[String], profile: &Profile) -> Vec<JoinHandle<()>> {
	extensions
		.iter()
		.map(|ext| {
			let ext_clone = ext.clone();
			let profile_clone = profile.clone();
			tokio::task::spawn(async move {
				install_extension_task(&ext_clone, &profile_clone).await;
			})
		})
		.collect()
}

pub async fn install_extension_task(extension: &str, profile: &Profile) {
	match query::query_extension(&extension).await {
		Ok(ext) => {
			println!("Installing extension {}.", extension);
			match install::install_extension(&ext, &profile).await {
				Ok(_) => println!("Successfully installed extension {}.", extension),
				Err(error) => eprintln!("Error installing extension {}. Error: {}", extension, error),
			};
		}

		Err(error) => {
			eprintln!("Error: {}", error);
		}
	}
}

pub async fn execute_tasks(tasks: Vec<JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}
