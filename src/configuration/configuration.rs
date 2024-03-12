use super::profile::Profile;
use crate::addons_json_database::addons_json_database::AddonsJsonDatabase;
use crate::errors::Error;
use crate::errors::Result;
use crate::extensions_json_database::extensions_json_database::ExtensionsJsonDatabase;

pub struct Configuration {
	pub profile: Profile,
	pub addons_json_database: AddonsJsonDatabase,
	pub extensions_json_database: ExtensionsJsonDatabase,
}

impl TryFrom<crate::cli::Configuration> for Configuration {
	type Error = Error;

	fn try_from(configuration: crate::cli::Configuration) -> Result<Self> {
		let profile = Profile::try_from(configuration)?;
		let extensions_json_database = ExtensionsJsonDatabase::try_from(&profile)?;
		let addons_json_database = AddonsJsonDatabase::try_from(&profile)?;

		Ok(Self {
			profile,
			extensions_json_database,
			addons_json_database,
		})
	}
}
