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
	Delete {
		#[arg(num_args = 1.., required = true)]
		delete: Vec<String>,
	},

	Install {
		#[arg(num_args = 1.., required = true)]
		install: Vec<String>,
	},

	List,

	Search {
		#[arg(required = true)]
		search: String,
	},

	Update {
		#[arg(num_args = 1..)]
		update: Option<Vec<String>>,
	},
}
