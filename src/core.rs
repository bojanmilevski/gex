use crate::extension::Extension;
use crate::flags::profile::Profile;
use crate::flags::Extensions;
use crate::install;

use colored::Colorize;
use tokio::task::JoinHandle;

pub fn create_install_tasks(extensions: &Extensions, profile: &Profile) -> Vec<JoinHandle<()>> {
	extensions
		.extensions
		.iter()
		.map(|ext| {
			let ext_clone = ext.clone();
			let profile_clone = profile.clone();
			tokio::task::spawn(async move {
				install_extension_task(ext_clone, &profile_clone).await;
			})
		})
		.collect()
}

pub async fn install_extension_task(extension: Extension, profile: &Profile) {
	println!("{}", "Installing extension".bold().green());
	println!("{}", extension);
	let name = extension.clone().name.name.unwrap_or("EMPTY".to_string());

	match install::install_extension(&extension, &profile).await {
		Ok(_) => {
			println!("{} {}.", "Successfully installed extension".bold().green(), name);
		}

		Err(error) => {
			eprintln!("{}: {}.", "Error installing extension".bold().red(), name);
			eprintln!("Error: {}", error);
		}
	};
}

pub async fn execute_tasks(tasks: Vec<JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}
