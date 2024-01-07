use super::configurable::Configurable;
use crate::cli::Cli;

pub struct Update {}

impl Configurable for Update {
	async fn try_configure_from(cli: &Cli) -> crate::errors::Result<Self> {
		Ok(Self {})
	}
}
