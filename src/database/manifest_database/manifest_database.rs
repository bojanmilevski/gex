use super::manifest::Manifest;
use crate::addon::addon::Addon;
use crate::database::extensions_json_database::extensions_json_database::ExtensionsJsonDatabase;
use crate::errors::Error;
use crate::errors::Result;

pub struct ManifestDatabase {
	pub manifests: Vec<Manifest>,
}

impl TryFrom<&ExtensionsJsonDatabase> for ManifestDatabase {
	type Error = Error;

	fn try_from(db: &ExtensionsJsonDatabase) -> Result<Self> {
		let manifests = db
			.addons
			.iter()
			.map(|addon| Manifest::try_from(addon).unwrap())
			.collect();

		Ok(Self { manifests })
	}
}

impl ManifestDatabase {
	pub fn add(&mut self, bytes: &Vec<u8>) -> Result<()> {
		let manifest = Manifest::try_from(bytes)?;
		self.manifests.push(manifest);
		Ok(())
	}

	pub fn delete(&mut self, addon: &Addon) -> Result<()> {
		let index = self
			.manifests
			.iter()
			.position(|manifest| manifest.browser_specific_settings.gecko.id == addon.guid)
			.unwrap();

		self.manifests.remove(index);

		Ok(())
	}
}
