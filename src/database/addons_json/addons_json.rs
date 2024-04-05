use super::addon::AddonsJsonAddon;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddonsJson {
	schema: u8,
	pub addons: Vec<AddonsJsonAddon>,
}

impl TryFrom<&Profile> for AddonsJson {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let path = profile.path.join("addons.json");
		let content = std::fs::read_to_string(path)?;
		let addons: AddonsJson = serde_json::from_str(&content)?;

		Ok(addons)
	}
}

impl AddonsJson {
	pub fn get(&self) -> Vec<String> {
		self.addons
			.iter()
			.map(|addon| addon.get_slug().unwrap())
			.collect()
	}

	pub fn add(&mut self, addon: &Addon) -> Result<()> {
		let addon = AddonsJsonAddon::try_from(addon)?;
		self.addons.push(addon);
		Ok(())
	}

	pub fn delete(&mut self, addon: &Addon) -> Result<()> {
		let index = self
			.addons
			.iter()
			.position(|addons_json_addon| addons_json_addon.get_slug().unwrap() == addon.slug.clone().unwrap())
			.unwrap();

		self.addons.remove(index);

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("addons.json"), content)?;

		Ok(())
	}
}
