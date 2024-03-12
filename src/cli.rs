use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(subcommand)]
	pub operation: Operation,

	#[arg(short, long)]
	pub verbose: bool,

	#[arg(short, long)]
	pub log: Option<PathBuf>,

	#[arg(short, long)]
	pub debug: bool,
}

#[derive(Parser)]
pub struct Configuration {
	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long)]
	pub profile: Option<String>,
}

#[derive(Subcommand)]
pub enum Operation {
	Delete {
		#[arg(num_args = 1.., required = true)]
		delete: Vec<String>,

		#[command(flatten)]
		configuration: Configuration,
	},

	Install {
		#[arg(num_args = 1.., required = true)]
		install: Vec<String>,

		#[command(flatten)]
		configuration: Configuration,
	},

	List {
		#[command(flatten)]
		configuration: Configuration,
	},

	Search {
		#[arg(required = true)]
		search: String,
	},

	Update {
		#[arg(num_args = 1..)]
		update: Option<Vec<String>>,

		#[command(flatten)]
		configuration: Configuration,
	},
}
