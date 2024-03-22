use crate::cli::Configuration as CliConfiguration;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub struct Delete {}

impl Delete {
	pub async fn try_configure_from(_val: Vec<String>, _configuration: CliConfiguration) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Delete {
	async fn try_run(&mut self) -> Result<()> {
		todo!()
	}
}
