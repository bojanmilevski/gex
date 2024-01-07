mod api_url;
mod cli;
mod core;
mod errors;
mod extension;
mod flags;
mod install;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use errors::Result;
use flags::configurable::Configurable;
use flags::flags::Flags;

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();
	let flags = Flags::try_configure_from(&cli).await?;

	if cli.operation.search.is_none() {
		let tasks = core::create_install_tasks(flags);
		core::execute_tasks(tasks).await;
	} else {
		for extension in flags.search.extensions {
			println!("{}", extension);
		}
	}

	/*
	// for manipulating the extensions.json file
	  let path = flags.profile.path.join("extensions.json");
	  let file = std::fs::File::open(&path)?;
	  let reader = BufReader::new(&file);
	  let content: Value = serde_json::from_reader(reader)?;
	  println!("{:#?}", content);
	  */

	Ok(())
}
