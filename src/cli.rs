use clap::Args;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(flatten)]
	pub operation: Operation,

	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long)]
	pub profile: Option<String>,

	#[arg(short, long)]
	pub verbose: bool,

	#[arg(short = 'o', long)]
	pub log: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Operation {
	#[arg(short = 'i', long = "install", num_args = 1..)]
	pub extensions: Vec<String>,

	#[arg(short, long)]
	pub search: Option<String>,

	#[arg(short, long)]
	pub delete: Vec<String>,

	#[arg(short, long)]
	pub update: Option<Vec<String>>,

	#[arg(short, long)]
	pub list: bool,
}
