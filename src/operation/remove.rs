use crate::cli::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub struct Remove {
	ids: Vec<String>,
	configuration: Configuration,
}

impl Remove {
	pub async fn try_configure_from(slugs: Vec<String>, configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(configuration)?;
		let ids = configuration
			.database
			.get_addons(Some(slugs))?
			.iter()
			.map(|(_, id, _)| id.clone())
			.collect();

		Ok(Self { ids, configuration })
	}
}

impl Runnable for Remove {
	async fn try_run(&mut self) -> Result<()> {
		let ids: Vec<&str> = self.ids.iter().map(|id| id.as_ref()).collect(); // FIX:

		self.configuration.database.remove_from_database(&ids)?;

		self.configuration
			.database
			.remove_from_disk(&ids, &self.configuration.profile)?;

		self.configuration
			.database
			.write(&self.configuration.profile)?;

		Ok(())
	}
}
