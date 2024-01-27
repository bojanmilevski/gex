use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Log {}

impl Configurable for Log {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		Ok(Self {})
	}
}
