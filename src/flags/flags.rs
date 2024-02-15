use crate::cli::Cli;
use crate::errors::Result;
use crate::operation::configurable::Configurable;
use crate::operation::operation::Operation;
use crate::operation::runnable::Runnable;
use std::path::PathBuf;

pub struct Flags {
	_debug: bool,
	_log: Option<PathBuf>,
	operation: Operation,
	_verbose: bool,
}

impl Configurable for Flags {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		Ok(Self {
			operation: Operation::try_configure_from(cli.clone()).await?,
			_verbose: cli.verbose,
			_log: cli.log,
			_debug: cli.debug,
		})
	}
}

impl Runnable for Flags {
	async fn try_run(&self) -> Result<()> {
		self.operation.try_run().await?;
		Ok(())
	}
}
