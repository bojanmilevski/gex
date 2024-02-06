use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(subcommand)]
	pub operation: Operation,

	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long)]
	pub profile: Option<String>,

	#[arg(short, long)]
	pub verbose: bool,

	#[arg(short, long)]
	pub log: Option<PathBuf>,

	#[arg(short, long)]
	pub debug: bool,
}

#[derive(Subcommand, Clone)]
pub enum Operation {
	DELETE { delete: Vec<String> },
	INSTALL { install: Vec<String> },
	LIST,
	SEARCH { search: String },
	UPDATE { update: Option<Vec<String>> },
}
