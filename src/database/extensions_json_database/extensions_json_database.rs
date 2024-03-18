use super::addon::addon::Addon as SelfAddon;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::traits::crud::CRUD;
use crate::traits::deserializeable::Deserializable;
use crate::traits::serializeable::Serializable;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsJsonDatabase {
	pub addons: Vec<SelfAddon>,
}

impl TryFrom<&Profile> for ExtensionsJsonDatabase {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let extensions_json_path = profile.path.join("extensions.json");
		let database = std::fs::File::open(extensions_json_path)?;
		let reader = BufReader::new(database);
		let mut addons: ExtensionsJsonDatabase = serde_json::from_reader(reader)?;

		// TODO: write deserializer
		addons
			.addons
			.retain(|addon| addon.location != "app-builtin" && addon.location != "app-system-defaults");

		Ok(Self { addons: addons.addons })
	}
}

impl Serializable for ExtensionsJsonDatabase {
	fn serialize(&self) -> Result<()> {
		todo!()
	}
}

impl Deserializable for ExtensionsJsonDatabase {
	fn deserialize(&self) -> Result<()> {
		todo!()
	}
}

impl CRUD for ExtensionsJsonDatabase {
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
