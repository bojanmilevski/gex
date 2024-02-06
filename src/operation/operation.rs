use super::configurable::Configurable;
use super::delete::Delete;
use super::install::Install;
use super::list::List;
use super::runnable::Runnable;
use super::search::Search;
use super::update::Update;
use crate::cli::Cli;
use crate::cli::Operation as Op;
use crate::configuration::profile::Profile;
use crate::errors::Result;

pub enum Operation {
	DELETE(Delete),
	INSTALL(Install),
	LIST(List),
	SEARCH(Search),
	UPDATE(Update),
}

impl Configurable for Operation {
	async fn try_configure_from(cli: Cli, profile: Profile) -> Result<Self> {
		let operation = match cli.operation {
			Op::DELETE { delete } => Self::DELETE(Delete::try_configure_from(delete, profile).await?),
			Op::INSTALL { install } => Self::INSTALL(Install::try_configure_from(install, profile).await?),
			Op::LIST => Self::LIST(List::try_configure_from(profile).await?),
			Op::SEARCH { search } => Self::SEARCH(Search::try_configure_from(search, profile).await?),
			Op::UPDATE { update } => Self::UPDATE(Update::try_configure_from(update, profile).await?),
		};

		Ok(operation)
	}
}

impl Runnable for Operation {
	async fn try_run(&self) -> Result<()> {
		match &self {
			Self::DELETE(delete) => delete.try_run().await?,
			Self::INSTALL(install) => install.try_run().await?,
			Self::LIST(list) => list.try_run().await?,
			Self::SEARCH(search) => search.try_run().await?,
			Self::UPDATE(update) => update.try_run().await?,
		}

		Ok(())
	}
}
