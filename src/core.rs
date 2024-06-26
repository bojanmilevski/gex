use crate::cli::cli::Cli;
use crate::operation::operation::Operation;
use crate::traits::configurable::Configurable;
use crate::traits::runnable::Runnable;
use anyhow::Result;

pub struct Core {
	operation: Operation,
}

impl Configurable for Core {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		let operation = Operation::try_configure_from(cli.operation).await?;

		Ok(Self { operation })
	}
}

impl Runnable for Core {
	async fn try_run(&mut self) -> Result<()> {
		self.operation.try_run().await?;
		Ok(())
	}
}
