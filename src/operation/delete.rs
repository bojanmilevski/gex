use super::runnable::Runnable;
use crate::configuration::profile::Profile;
use crate::errors::Result;

pub struct Delete {}

impl Delete {
	pub async fn try_configure_from(val: Vec<String>, profile: Profile) -> Result<Self> {
		Ok(Self {})
	}
}

impl Runnable for Delete {
	async fn try_run(&self) -> Result<()> {
		todo!()
	}
}
