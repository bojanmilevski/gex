use crate::cli::Cli;
use crate::configuration::profile::Profile;
use crate::errors::Result;
use crate::operation::configurable::Configurable;
use crate::operation::operation::Operation;
use crate::operation::runnable::Runnable;
use std::path::PathBuf;

pub struct Flags {
	operation: Operation,
	verbose: bool,
	log: Option<PathBuf>,
	debug: bool,
}

impl Configurable for Flags {
	async fn try_configure_from(cli: Cli, profile: Profile) -> Result<Self> {
		Ok(Self {
			operation: Operation::try_configure_from(cli.clone(), profile).await?,
			verbose: cli.verbose,
			log: cli.log,
			debug: cli.debug,
		})
	}
}

impl Runnable for Flags {
	async fn try_run(&self) -> Result<()> {
		self.operation.try_run().await?;
		Ok(())
	}
}
