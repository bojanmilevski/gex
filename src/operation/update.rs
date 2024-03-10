use crate::errors::Result;
use crate::runnable::Runnable;

pub struct Update {}

impl Update {
	pub async fn try_configure_from(
		_val: Option<Vec<String>>,
		_configuration: crate::cli::Configuration,
	) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Update {
	async fn try_run(&self) -> Result<()> {
		Ok(())
	}
}
