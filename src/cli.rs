use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(subcommand)]
	pub operation: CliOperation,
}

#[derive(Subcommand)]
pub enum CliOperation {
	#[clap(visible_alias = "i")]
	Install {
		#[command(flatten)]
		configuration: CliConfiguration,

		#[arg(name = "install", num_args = 1.., required = true)]
		slugs: Vec<String>,
	},

	#[clap(visible_alias = "ls")]
	List {
		#[command(flatten)]
		configuration: CliConfiguration,
	},

	#[clap(alias = "delete", visible_alias = "rm")]
	Remove {
		#[command(flatten)]
		configuration: CliConfiguration,

		#[arg(name = "remove", num_args = 1.., required = true)]
		slugs: Vec<String>,
	},

	#[clap(visible_alias = "s")]
	Search {
		#[arg(name = "search", required = true)]
		slug: String,
	},

	#[clap(visible_alias = "u")]
	Update {
		#[command(flatten)]
		configuration: CliConfiguration,

		#[arg(name = "update", num_args = 1..)]
		slugs: Option<Vec<String>>,
	},
}

#[derive(Parser)]
pub struct CliConfiguration {
	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long)]
	pub profile: Option<String>,
}
