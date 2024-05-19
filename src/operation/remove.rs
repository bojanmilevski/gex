use crate::cli::configuration::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::traits::runnable::Runnable;
use anyhow::Result;

pub struct Remove {
	slugs: Vec<String>,
	configuration: Configuration,
}

// FIX: configurable trait
impl Remove {
	pub async fn try_configure_from(slugs: Vec<String>, configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(configuration)?;
		let slugs = configuration
			.database
			.get_addons(Some(slugs))?
			.iter()
			.map(|(slug, _, _)| slug.clone())
			.collect();

		Ok(Self { slugs, configuration })
	}
}

impl Runnable for Remove {
	async fn try_run(&mut self) -> Result<()> {
		let slugs: Vec<&str> = self.slugs.iter().map(|slug| slug.as_ref()).collect(); // FIX:

		let removed = self.configuration.database.remove_from_database(&slugs)?;

		self.configuration
			.database
			.remove_from_disk(removed, &self.configuration.profile)?;

		self.configuration
			.database
			.write(&self.configuration.profile)?;

		Ok(())
	}
}
