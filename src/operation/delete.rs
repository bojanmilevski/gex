use super::runnable::Runnable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Delete {}

impl Delete {
	pub async fn try_configure_from(_val: Vec<String>, _cli: Cli) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Delete {
	async fn try_run(&self) -> Result<()> {
		todo!()
	}
}
