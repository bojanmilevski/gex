use super::delete::Delete;
use super::install::Install;
use super::list::List;
use super::search::Search;
use super::update::Update;
use crate::cli::Operation as Op;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub enum Operation {
	Delete(Delete),
	Install(Install),
	List(List),
	Search(Search),
	Update(Update),
}

impl Operation {
	pub async fn try_configure_from(operation: crate::cli::Operation) -> Result<Self> {
		let operation = match operation {
			Op::Delete { delete, configuration } => {
				Self::Delete(Delete::try_configure_from(delete, configuration).await?)
			}

			Op::Install { install, configuration } => {
				Self::Install(Install::try_configure_from(install, configuration).await?)
			}

			Op::List { configuration } => Self::List(List::try_configure_from(configuration).await?),

			Op::Search { search } => Self::Search(Search::try_configure_from(search).await?),

			Op::Update { update, configuration } => {
				Self::Update(Update::try_configure_from(update, configuration).await?)
			}
		};

		Ok(operation)
	}
}

impl Runnable for Operation {
	async fn try_run(&mut self) -> Result<()> {
		match self {
			Self::Delete(delete) => delete.try_run().await?,
			Self::Install(install) => install.try_run().await?,
			Self::List(list) => list.try_run().await?,
			Self::Search(search) => search.try_run().await?,
			Self::Update(update) => update.try_run().await?,
		}

		Ok(())
	}
}
