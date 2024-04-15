use super::addon::addon::AddonsJsonAddon;
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
	pub fn add(&mut self, addon_map: &[(&Addon, Vec<u8>)]) -> Result<()> {
		let new_addons = addon_map
			.iter()
			.map(|(addon, _)| AddonsJsonAddon::try_from(*addon))
			.collect::<Result<Vec<AddonsJsonAddon>>>()?;

		new_addons
			.into_iter()
			.for_each(|addon| self.addons.push(addon));

		Ok(())
	}

	pub fn remove(&mut self, ids: &[&str]) -> Result<()> {
		ids.iter().for_each(|id| {
			let index = self
				.addons
				.iter()
				.position(|addon| &addon.id == id)
				.ok_or(crate::errors::Error::Home)
				.unwrap();

			self.addons.remove(index);
		});

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("addons.json"), content)?;

		Ok(())
	}
}
