use crate::flags::flags::Flags;
use crate::install;
use colored::Colorize;
use tokio::task::JoinHandle;

pub fn create_install_tasks(flags: &Flags) -> Vec<JoinHandle<()>> {
	flags
		.install
		.extensions
		.iter()
		.map(|ext| {
			let ext = ext.clone();
			let profile = flags.profile.clone();
			tokio::task::spawn(async move {
				println!("{}", ext);
				if let Err(err) = install::install_extension(ext, profile).await {
					eprintln!("{}: {}.", "Error installing extension".bold().red(), "TODO");
					eprintln!("{}: {}", "Error".bold().red(), err);
				};
			})
		})
		.collect()
}

pub async fn execute_tasks(tasks: Vec<JoinHandle<()>>) {
	for task in tasks {
		let _ = task.await;
	}
}
