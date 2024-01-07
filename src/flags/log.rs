use super::configurable::Configurable;
use crate::cli::Cli;

pub struct Log {}

impl Configurable for Log {
	async fn try_configure_from(cli: &Cli) -> crate::errors::Result<Self> {
		Ok(Self {})
	}
}
