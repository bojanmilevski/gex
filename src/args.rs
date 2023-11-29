use crate::browser::Browser;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true, conflicts_with = "query")]
	pub install: Vec<String>,

	#[arg(short, long, default_value = "default-release")]
	pub profile: String,

	#[arg(short, long, default_value = "firefox")]
	pub browser: Browser,

	#[arg(short, long, required = false, default_value = "", conflicts_with = "install")]
	pub query: String,
}
