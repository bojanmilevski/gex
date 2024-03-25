use super::addon::addon::ExtensionsJsonAddon;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsJson {
	schema_version: u8,
	pub addons: Vec<ExtensionsJsonAddon>,
}

impl TryFrom<&Profile> for ExtensionsJson {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let path = profile.path.join("extensions.json");
		let content = std::fs::read_to_string(path)?;
		let addons: ExtensionsJson = serde_json::from_str(&content)?;

		Ok(addons)
	}
}

impl ExtensionsJson {
	pub fn add(&mut self, bytes: &Vec<u8>, manifest: &Manifest, profile: &Profile) -> Result<()> {
		let extensions_json_addon = ExtensionsJsonAddon::try_from((bytes, manifest, profile))?;
		self.addons.push(extensions_json_addon);
		Ok(())
	}

	pub fn delete(&mut self, addon: &Addon) -> Result<()> {
		let index = self
			.addons
			.iter()
			.position(|extensions_json_addon| extensions_json_addon.sync_guid.clone().unwrap() == addon.guid)
			.unwrap();

		self.addons.remove(index);

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("extensions.json"), content)?;

		Ok(())
	}
}
