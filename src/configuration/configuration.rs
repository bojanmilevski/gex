use super::profile::Profile;
use crate::cli::CliConfiguration;
use crate::database::database::Database;
use crate::errors::Error;
use crate::errors::Result;

pub struct Configuration {
	pub database: Database,
	pub profile: Profile,
}

impl TryFrom<CliConfiguration> for Configuration {
	type Error = Error;

	fn try_from(configuration: CliConfiguration) -> Result<Self> {
		let profile = Profile::try_from(configuration)?;
		let database = Database::try_from(&profile)?;

		Ok(Self { database, profile })
	}
}
