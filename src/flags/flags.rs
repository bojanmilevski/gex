use super::browser::Browser;
use super::configurable::Configurable;
use super::delete::Delete;
use super::install::Install;
use super::list::List;
use super::log::Log;
use super::profile::Profile;
use super::runnable::Runnable;
use super::search::Search;
use super::update::Update;
use super::verbose::Verbose;
use crate::cli::Cli;
use crate::errors::Result;

pub struct Flags {
	pub browser: Browser,
	pub delete: Delete,
	pub install: Install,
	pub list: List,
	pub log: Log,
	pub profile: Profile,
	pub search: Search,
	pub update: Update,
	pub verbose: Verbose,
}

impl Configurable for Flags {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		Ok(Self {
			browser: Browser::try_configure_from(&cli).await?,
			delete: Delete::try_configure_from(&cli).await?,
			install: Install::try_configure_from(&cli).await?,
			list: List::try_configure_from(&cli).await?,
			log: Log::try_configure_from(&cli).await?,
			profile: Profile::try_configure_from(&cli).await?,
			search: Search::try_configure_from(&cli).await?,
			update: Update::try_configure_from(&cli).await?,
			verbose: Verbose::try_configure_from(&cli).await?,
		})
	}
}

impl Runnable for Flags {
	async fn try_run(&self, flags: &Flags) -> Result<()> {
		if !flags.install.extensions.is_empty() {
			flags.install.try_run(&flags).await?;
		} else if !flags.search.extensions.is_empty() {
			flags.search.try_run(&flags).await?;
		} else if !flags.update.update.extensions.is_empty() {
			flags.update.try_run(&flags).await?;
		}

		Ok(())
	}
}
