use super::install::Install;
use super::list::List;
use super::remove::Remove;
use super::search::Search;
use super::update::Update;
use crate::cli::CliOperation;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub enum Operation {
	Remove(Remove),
	Install(Install),
	List(List),
	Search(Search),
	Update(Update),
}

impl Operation {
	// FIX: .dedup()
	pub async fn try_configure_from(operation: CliOperation) -> Result<Self> {
		let operation = match operation {
			CliOperation::Remove { slugs, configuration } => {
				let mut slugs = slugs.clone();
				slugs.dedup();

				Self::Remove(Remove::try_configure_from(slugs, configuration).await?)
			}

			CliOperation::Install { slugs, configuration } => {
				let mut slugs = slugs.clone();
				slugs.dedup();

				Self::Install(Install::try_configure_from(slugs, configuration).await?)
			}

			CliOperation::List { configuration } => Self::List(List::try_configure_from(configuration).await?),

			CliOperation::Search { slug } => Self::Search(Search::try_configure_from(slug).await?),

			CliOperation::Update { slugs, configuration } => {
				if slugs.is_some() {
					let slugs = slugs.clone();
					slugs.unwrap().dedup();
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
