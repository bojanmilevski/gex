use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(subcommand)]
	pub operation: Operation,
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
	#[clap(alias = "remove", visible_alias = "rm")]
	Delete {
		#[command(flatten)]
		configuration: Configuration,

		#[arg(num_args = 1.., required = true)]
		delete: Vec<String>,
	},

	#[clap(visible_alias = "i")]
	Install {
		#[command(flatten)]
		configuration: Configuration,

		#[arg(num_args = 1.., required = true)]
		install: Vec<String>,
	},

	#[clap(visible_alias = "ls")]
	List {
		#[command(flatten)]
		configuration: Configuration,
	},

	#[clap(visible_alias = "s")]
	Search {
		#[arg(required = true)]
		search: String,
	},

	#[clap(visible_alias = "u")]
	Update {
		#[command(flatten)]
		configuration: Configuration,

		#[arg(num_args = 1..)]
		update: Option<Vec<String>>,
	},
}
