use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct Args {
	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short = 'i', long = "install", num_args = 1.., value_delimiter = ' ', conflicts_with = "search")]
	pub extensions: Vec<String>,

	#[arg(short, long)]
	pub profile: Option<String>,

	#[arg(short, long, conflicts_with = "extensions")]
	pub search: Option<String>,

	#[arg(short, long)]
	pub verbose: bool,
}
