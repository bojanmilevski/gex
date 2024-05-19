use clap::Parser;

#[derive(Parser)]
pub struct CliConfiguration {
	#[arg(short, long, default_value = "firefox")]
	pub browser: String,

	#[arg(short, long)]
	pub profile: Option<String>,
}
