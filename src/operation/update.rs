use crate::cli::Configuration as CliConfiguration;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub struct Update {}

impl Update {
	pub async fn try_configure_from(_val: Option<Vec<String>>, _configuration: CliConfiguration) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Update {
	async fn try_run(&mut self) -> Result<()> {
		Ok(())
	}
}
