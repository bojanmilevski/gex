use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Delete {}

impl Configurable for Delete {
	async fn try_configure_from(_cli: &Cli) -> Result<Self> {
		Ok(Self {})
	}
}
