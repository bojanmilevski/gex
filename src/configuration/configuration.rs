use super::profile::Profile;
use crate::cli::CliConfiguration;
use crate::database::database::Database;
use anyhow::Result;

pub struct Configuration {
	pub database: Database,
	pub profile: Profile,
}

impl TryFrom<CliConfiguration> for Configuration {
	type Error = anyhow::Error;

	fn try_from(configuration: CliConfiguration) -> Result<Self> {
		let profile = Profile::try_from(configuration)?;
		let database = Database::try_from(&profile)?;

		Ok(Self { database, profile })
	}
}
