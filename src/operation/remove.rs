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
		let ids = slugs
			.iter()
			.map(|slug| {
				configuration
					.database
					.addons_json_database
					.addons
					.iter()
					.find(|addon| &addon.slug() == slug)
					.unwrap()
					.id
					.to_owned()
			})
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
