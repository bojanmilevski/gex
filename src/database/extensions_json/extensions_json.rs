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
	pub fn add(&mut self, addon_map: &[(&Addon, Vec<u8>)], manifest: &Manifest, profile: &Profile) -> Result<()> {
		let new = addon_map
			.iter()
			.map(|addon| ExtensionsJsonAddon::try_from((addon, manifest, profile)))
			.collect::<Result<Vec<ExtensionsJsonAddon>>>()?;

		new.into_iter().for_each(|addon| self.addons.push(addon));
		Ok(())
	}

	pub fn remove(&mut self, ids: &[&str]) -> Result<()> {
		ids.iter().for_each(|id| {
			let index = self
				.addons
				.iter()
				.position(|addon| &addon.id == id)
				.unwrap();

			self.addons.remove(index);
		});

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("extensions.json"), content)?;

		Ok(())
	}
}
