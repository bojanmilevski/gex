use super::configurable::Configurable;
use crate::cli::Cli;

pub struct Delete {}

impl Configurable for Delete {
	async fn try_configure_from(cli: &Cli) -> crate::errors::Result<Self> {
		Ok(Self {})
	}
}
