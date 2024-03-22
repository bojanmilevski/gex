use super::addons_json_database::addons_json_database::AddonsJsonDatabase;
use super::extensions_json_database::extensions_json_database::ExtensionsJsonDatabase;
use super::manifest_database::manifest_database::ManifestDatabase;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;

pub struct Database {
	pub addons_json_database: AddonsJsonDatabase,
	pub extensions_json_database: ExtensionsJsonDatabase,
	manifest_database: ManifestDatabase,
}

impl TryFrom<&Profile> for Database {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let addons_json_database = AddonsJsonDatabase::try_from(profile)?;
		let extensions_json_database = ExtensionsJsonDatabase::try_from(profile)?;
		let manifest_database = ManifestDatabase::try_from(&extensions_json_database)?;

		Ok(Self { addons_json_database, extensions_json_database, manifest_database })
	}
}

impl Database {
	pub fn get(&self) -> Vec<String> {
		self.addons_json_database.get()
	}

	pub fn add(&mut self, addon: &Addon, bytes: &Vec<u8>, profile: &Profile) -> Result<()> {
		self.addons_json_database.add(addon)?;
		self.manifest_database.add(bytes)?;
		self.extensions_json_database
			.add(bytes, self.manifest_database.manifests.last().unwrap(), profile)?;

		Ok(())
	}

	pub fn delete(&mut self, addon: &Addon) -> Result<()> {
		self.addons_json_database.delete(addon)?;
		self.extensions_json_database.delete(addon)?;
		self.manifest_database.delete(addon)?;

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		self.extensions_json_database.write(profile)?;
		self.addons_json_database.write(profile)?;

		Ok(())
	}
}
