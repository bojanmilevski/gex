use super::operation::CliOperation;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[command(subcommand)]
	pub operation: CliOperation,
}
