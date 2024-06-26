use super::super::extensions_json::extensions_json::ExtensionsJson;
use super::manifest::Manifest;
use crate::addon::addon::Addon;
use anyhow::Result;

pub struct Manifests {
	pub manifests: Vec<Manifest>,
}

impl TryFrom<&ExtensionsJson> for Manifests {
	type Error = anyhow::Error;

	fn try_from(db: &ExtensionsJson) -> Result<Self> {
		let manifests = db
			.addons
			.iter()
			.filter(|addon| addon.is_not_builtin())
			.map(|addon| Manifest::try_from(addon).unwrap())
			.collect();

		Ok(Self { manifests })
	}
}

impl Manifests {
	pub fn add(&mut self, addon_map: &[(&Addon, Vec<u8>)]) -> Result<()> {
		let new = addon_map
			.iter()
			.map(|(_, bytes)| Manifest::try_from(bytes))
			.collect::<Result<Vec<Manifest>>>()?;

		new.into_iter()
			.for_each(|manifest| self.manifests.push(manifest));

		Ok(())
	}

	pub fn remove(&mut self, ids: &[String]) -> Result<()> {
		ids.iter().for_each(|id| {
			let index = self
				.manifests
				.iter()
				.position(|manifest| {
					if manifest.browser_specific_settings.is_some() {
						manifest
							.browser_specific_settings
							.as_ref()
							.unwrap()
							.gecko
							.id == id.as_ref()
					} else {
						manifest.applications.as_ref().unwrap().gecko.id == id.as_ref()
					}
				})
				.unwrap();

			self.manifests.remove(index);
		});

		Ok(())
	}
}
