use crate::extension::Extension;
use crate::flags::profile::Profile;
use crate::flags::Flags;
use crate::install;
use colored::Colorize;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use tokio::task::JoinHandle;

pub fn create_install_tasks(flags: &Flags) -> Vec<JoinHandle<()>> {
	flags
		.extensions
		.extensions
		.par_iter()
		.map(|ext| {
			let ext_clone = ext.clone();
			let profile_clone = flags.profile.clone();
			tokio::task::spawn(async move {
				install_extension_task(ext_clone, &profile_clone).await;
			})
		})
		.collect()
}

pub async fn install_extension_task(extension: Extension, profile: &Profile) {
	println!("{}", "Installing extension".bold().green());
	println!("{}", extension);
	let name = extension.clone().name.name.unwrap_or("EMPTY".to_string()); // stupid but works for now

	match install::install_extension(&extension, &profile).await {
		Ok(_) => {
			println!("{} {}.", "Successfully installed extension".bold().green(), name);
		}

		Err(err) => {
			eprintln!("{}: {}.", "Error installing extension".bold().red(), name);
			eprintln!("Error: {}", err);
		}
	};
}

pub async fn execute_tasks(tasks: Vec<JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}