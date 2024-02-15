use super::runnable::Runnable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Update {}

impl Update {
	pub async fn try_configure_from(_val: Option<Vec<String>>, _cli: Cli) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Update {
	async fn try_run(&self) -> Result<()> {
		Ok(())
	}
}
