use super::profile::Profile;
use crate::database::database::Database;
use crate::errors::Error;
use crate::errors::Result;

pub struct Configuration {
	pub database: Database,
	pub profile: Profile,
}

impl TryFrom<crate::cli::Configuration> for Configuration {
	type Error = Error;

	fn try_from(configuration: crate::cli::Configuration) -> Result<Self> {
		let profile = Profile::try_from(configuration)?;
		let database = Database::try_from(&profile)?;

		Ok(Self { database, profile })
	}
}
