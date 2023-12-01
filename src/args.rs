use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true, conflicts_with = "search")]
	pub extensions: Vec<String>,

	#[arg(short, long, default_value = "default-release")]
	pub profile: String,

	#[arg(short, long, required = false, default_value = "", conflicts_with = "extensions")]
	pub search: String,
}
