use crate::extension::extension::Extension;
use crate::flags::flags::Flags;
use crate::flags::profile::Profile;
use crate::install;
use colored::Colorize;
use tokio::task::JoinHandle;

pub fn create_install_tasks(flags: &Flags) -> Vec<JoinHandle<()>> {
	flags
		.extensions
		.extensions
		.iter()
		.map(|ext| {
			let ext = ext.clone();
			let profile = flags.profile.clone();
			tokio::task::spawn(async move {
				install_extension_task(&ext, &profile).await;
			})
		})
		.collect()
}

async fn install_extension_task(extension: &Extension, profile: &Profile) {
	println!("{}", "Installing extension".bold().green());
	println!("{}", extension);
	let name = extension.clone().name.name.unwrap_or("EMPTY".to_string()); // stupid but works for now

	match install::install_extension(&extension, &profile).await {
		Ok(_) => {
			println!("{} {}.", "Successfully installed extension".bold().green(), name);
		}

		Err(err) => {
			eprintln!("{}: {}.", "Error installing extension".bold().red(), name);
			eprintln!("{}: {}", "Error".bold().red(), err);
		}
	};
}

pub async fn execute_tasks(tasks: Vec<JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}
