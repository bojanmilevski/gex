use super::configurable::Configurable;
use super::flags::Flags;
use super::install::Install;
use super::runnable::Runnable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Update {
	pub update: Install,
}

impl Configurable for Update {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		Ok(Self { update: Install::try_configure_from(&cli).await? })
	}
}

impl Runnable for Update {
	async fn try_run(&self, _flags: &Flags) -> Result<()> {
		todo!()
	}
}
