use crate::errors::Result;
use crate::runnable::Runnable;

pub struct Delete {}

impl Delete {
	pub async fn try_configure_from(
		_val: Vec<String>,
		_configuration: crate::cli::Configuration,
	) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Delete {
	async fn try_run(&self) -> Result<()> {
		todo!()
	}
}
