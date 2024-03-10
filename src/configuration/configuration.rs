use super::intermediate_database::IntermediateDatabase;
use super::profile::Profile;
use crate::database::database::Database;

pub struct Configuration {
	pub profile: Profile,
	pub intermediate_database: IntermediateDatabase,
	pub database: Database,
}

impl Configuration {
	pub async fn try_configure_from(
		configuration: crate::cli::Configuration,
	) -> crate::errors::Result<Self> {
		let profile = Profile::try_from(configuration)?;
		let database = Database::try_from(&profile)?;
		let intermediate_database = IntermediateDatabase::try_configure_from(&database).await?;

		Ok(Self {
			profile,
			database,
			intermediate_database,
		})
	}
}
