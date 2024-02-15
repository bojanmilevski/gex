use super::configurable::Configurable;
use super::runnable::Runnable;
use crate::cli::Cli;
use crate::configuration::profile::Profile;
use crate::errors::Result;

pub struct List {
	list: Vec<String>,
	profile: Profile,
}

impl List {
	pub async fn try_configure_from(cli: Cli) -> Result<Self> {
		let profile = Profile::try_configure_from(cli).await?;
		let list = profile.database.slugs.clone();
		Ok(Self { profile, list })
	}
}

impl Runnable for List {
	async fn try_run(&self) -> Result<()> {
		self.list.iter().for_each(|ext| println!("{}", ext));
		Ok(())
	}
}
