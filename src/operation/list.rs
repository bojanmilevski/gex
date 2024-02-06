use super::runnable::Runnable;
use crate::configuration::profile::Profile;
use crate::database::addon::Addons;
use crate::errors::Result;

pub struct List {
	list: Addons,
}

impl List {
	pub async fn try_configure_from(profile: Profile) -> Result<Self> {
		Ok(Self { list: Addons::try_from(&profile.path)? })
	}
}

impl Runnable for List {
	async fn try_run(&self) -> Result<()> {
		for extension in &self.list.addons {
			println!("{}", extension);
		}

		Ok(())
	}
}
