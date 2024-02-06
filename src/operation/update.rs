use super::install::Install;
use super::runnable::Runnable;
use crate::configuration::profile::Profile;
use crate::errors::Result;

pub struct Update {
	pub update: Install,
}

impl Update {
	pub async fn try_configure_from(val: Option<Vec<String>>, profile: Profile) -> Result<Self> {
		Ok(Self { update: Install::try_configure_from(val.unwrap(), profile).await? })
	}
}

impl Runnable for Update {
	async fn try_run(&self) -> Result<()> {
		Ok(self.update.try_run().await?)
	}
}
