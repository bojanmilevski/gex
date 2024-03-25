use crate::cli::Cli;
use crate::errors::Result;
use crate::operation::operation::Operation;
use crate::traits::configurable::Configurable;
use crate::traits::runnable::Runnable;

pub struct Core {
	operation: Operation,
}

impl Configurable for Core {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		Ok(Self { operation: Operation::try_configure_from(cli.operation).await? })
	}
}

impl Runnable for Core {
	async fn try_run(&mut self) -> Result<()> {
		self.operation.try_run().await?;
		Ok(())
	}
}
