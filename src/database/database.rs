use super::addons_json_database::addons_json_database::AddonsJsonDatabase;
use super::extensions_json_database::extensions_json_database::ExtensionsJsonDatabase;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::traits::crud::CRUD;
use crate::traits::deserializeable::Deserializable;
use crate::traits::serializeable::Serializable;

pub struct Database {
	pub addons_json_database: AddonsJsonDatabase,
	pub extensions_json_database: ExtensionsJsonDatabase,
}

impl TryFrom<&Profile> for Database {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let addons_json_database = AddonsJsonDatabase::try_from(profile)?;
		let extensions_json_database = ExtensionsJsonDatabase::try_from(profile)?;

		Ok(Self { addons_json_database, extensions_json_database })
	}
}

impl Serializable for Database {
	fn serialize(&self) -> Result<()> {
		todo!()
	}
}

impl Deserializable for Database {
	fn deserialize(&self) -> Result<()> {
		todo!()
	}
}

impl CRUD for Database {
	fn create() -> Result<()> {
		todo!()
	}

	fn read(&self) -> Result<()> {
		todo!()
	}

	fn update(&self, _addon: Addon) -> Result<()> {
		todo!()
	}

	fn delete(&self, _addon: Addon) -> Result<()> {
		todo!()
	}
}
