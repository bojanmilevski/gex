mod args;
mod core;
mod errors;
mod extension;
mod flags;
mod install;
mod progress_bar;
mod query;

use args::Args;
use clap::Parser;
use errors::Result;
use flags::configurable::Configurable;
use flags::flags::Flags;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let flags = Flags::configure_from(&args).await?;

	if args.search.is_none() {
		let tasks = core::create_install_tasks(&flags);
		core::execute_tasks(tasks).await;
	} else {
		for extension in flags.search.extensions {
			println!("{}", extension);
		}
	}

	/*
	let path = flags.profile.path.join("extensions.json");
	let file = std::fs::File::open(&path)?;
	let reader = BufReader::new(&file);
	let content: Value = serde_json::from_reader(reader)?;
	println!("{:#?}", content);
	*/

	Ok(())
}
