use crate::cli::Cli;
use crate::errors::Result;
use crate::operation::operation::Operation;
use crate::traits::configurable::Configurable;
use crate::traits::runnable::Runnable;
use std::path::PathBuf;

pub struct Core {
	_debug: bool,
	_log: Option<PathBuf>,
	operation: Operation,
	_verbose: bool,
}

impl Configurable for Core {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		Ok(Self {
			_debug: cli.debug,
			_log: cli.log,
			operation: Operation::try_configure_from(cli.operation).await?,
			_verbose: cli.verbose,
		})
	}
}

impl Runnable for Core {
	async fn try_run(&self) -> Result<()> {
		self.operation.try_run().await?;
		Ok(())
	}
}
