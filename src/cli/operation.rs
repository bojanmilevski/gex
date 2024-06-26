use super::configuration::CliConfiguration;
use clap::Subcommand;

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
