use super::install::Install;
use super::list::List;
use super::remove::Remove;
use super::search::Search;
use super::update::Update;
use crate::cli::CliOperation;
use crate::traits::runnable::Runnable;
use anyhow::Result;

pub enum Operation {
	Remove(Remove),
	Install(Install),
	List(List),
	Search(Search),
	Update(Update),
}

impl Operation {
	pub async fn try_configure_from(operation: CliOperation) -> Result<Self> {
		let operation = match operation {
			CliOperation::Remove { mut slugs, configuration } => {
				slugs.dedup();
				Self::Remove(Remove::try_configure_from(slugs, configuration).await?)
			}

			CliOperation::Install { mut slugs, configuration } => {
				slugs.dedup();
				Self::Install(Install::try_configure_from(slugs, configuration).await?)
			}

			CliOperation::List { configuration } => Self::List(List::try_configure_from(configuration).await?),

			CliOperation::Search { slug } => Self::Search(Search::try_configure_from(slug).await?),

			CliOperation::Update { mut slugs, configuration } => {
				if let Some(slugs) = &mut slugs {
					slugs.dedup();
				}

				Self::Update(Update::try_configure_from(slugs, configuration).await?)
			}
		};

		Ok(operation)
	}
}

impl Runnable for Operation {
	async fn try_run(&mut self) -> Result<()> {
		match self {
			Self::Remove(delete) => delete.try_run().await?,
			Self::Install(install) => install.try_run().await?,
			Self::List(list) => list.try_run().await?,
			Self::Search(search) => search.try_run().await?,
			Self::Update(update) => update.try_run().await?,
		}

		Ok(())
	}
}
