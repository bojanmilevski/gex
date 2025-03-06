use super::super::extensions_json::extensions_json::ExtensionsJson;
use super::manifest::Manifest;
use crate::operation::install::Package;
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
			.map(Manifest::try_from)
			.collect::<Result<Vec<Manifest>>>()?;

		Ok(Self { manifests })
	}
}

impl Manifests {
	pub fn add(&mut self, addons: &[Package]) -> Result<()> {
		self.manifests.extend(
			addons
				.iter()
				.map(|addon| Manifest::try_from(&addon.xpi))
				.collect::<Result<Vec<Manifest>>>()?,
		);

		Ok(())
	}

	pub fn remove(&mut self, addons: &[Package]) -> Result<()> {
		// TODO:

		Ok(())
	}
}
