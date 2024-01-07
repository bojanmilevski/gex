use super::configurable::Configurable;
use crate::cli::Cli;

pub struct Verbose {}

impl Configurable for Verbose {
	async fn try_configure_from(cli: &Cli) -> crate::errors::Result<Self> {
		Ok(Self {})
	}
}
