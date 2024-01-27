use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Verbose {}

impl Configurable for Verbose {
	async fn try_configure_from(_cli: &Cli) -> Result<Self> {
		Ok(Self {})
	}
}
