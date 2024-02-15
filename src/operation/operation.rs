use super::configurable::Configurable;
use super::delete::Delete;
use super::install::Install;
use super::list::List;
use super::runnable::Runnable;
use super::search::Search;
use super::update::Update;
use crate::cli::Cli;
use crate::cli::Operation as Op;
use crate::errors::Result;

pub enum Operation {
	Delete(Delete),
	Install(Install),
	List(List),
	Search(Search),
	Update(Update),
}

impl Configurable for Operation {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		let operation = match cli.clone().operation {
			Op::Delete { delete } => Self::Delete(Delete::try_configure_from(delete, cli).await?),
			Op::Install { install } => Self::Install(Install::try_configure_from(install, cli).await?),
			Op::List => Self::List(List::try_configure_from(cli).await?),
			Op::Search { search } => Self::Search(Search::try_configure_from(search).await?),
			Op::Update { update } => Self::Update(Update::try_configure_from(update, cli).await?),
		};

		Ok(operation)
	}
}

impl Runnable for Operation {
	async fn try_run(&self) -> Result<()> {
		match &self {
			Self::Delete(delete) => delete.try_run().await?,
			Self::Install(install) => install.try_run().await?,
			Self::List(list) => list.try_run().await?,
			Self::Search(search) => search.try_run().await?,
			Self::Update(update) => update.try_run().await?,
		}

		Ok(())
	}
}
