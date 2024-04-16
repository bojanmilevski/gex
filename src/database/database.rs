use super::addons_json::addons_json::AddonsJson;
use super::extensions_json::extensions_json::ExtensionsJson;
use super::manifests::manifests::Manifests;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use tokio::io::AsyncWriteExt;

// FIX: super database addon. vec<DatabaseAddon>
pub struct Database {
	pub addons_json_database: AddonsJson,
	pub extensions_json_database: ExtensionsJson,
	manifest_database: Manifests,
}

impl TryFrom<&Profile> for Database {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let addons_json_database = AddonsJson::try_from(profile)?;
		let extensions_json_database = ExtensionsJson::try_from(profile)?;
		let manifest_database = Manifests::try_from(&extensions_json_database)?;

		Ok(Self { addons_json_database, extensions_json_database, manifest_database })
	}
}

impl Database {
	pub fn get_slugs(&self) -> Vec<String> {
		self.addons_json_database
			.addons
			.iter()
			.map(|addon| addon.slug())
			.collect()
	}

	pub fn add(&mut self, addon_map: &[(&Addon, Vec<u8>)], profile: &Profile) -> Result<()> {
		self.addons_json_database.add(addon_map)?;
		self.manifest_database.add(addon_map)?;
		self.extensions_json_database
			.add(addon_map, self.manifest_database.manifests.last().unwrap(), profile)?;

		Ok(())
	}

	// FIX: slug instead of id
	pub fn remove_from_disk(&mut self, ids: &[&str], profile: &Profile) -> Result<()> {
		ids.iter().try_for_each(|id| {
			let path = profile.extensions.join(format!("{}.xpi", id));
			std::fs::remove_file(path)
		})?;

		Ok(())
	}

	// FIX: slug instead of id
	pub fn remove_from_database(&mut self, ids: &[&str]) -> Result<()> {
		self.addons_json_database.remove(ids)?;
		self.extensions_json_database.remove(ids)?;
		self.manifest_database.remove(ids)?;

		Ok(())
	}

	pub fn write(&self, profile: &Profile) -> Result<()> {
		self.extensions_json_database.write(profile)?;
		self.addons_json_database.write(profile)?;

		Ok(())
	}

	pub fn contains(&self, slug: &str) -> bool {
		self.addons_json_database
			.addons
			.iter()
			.map(|addon| addon.slug())
			.collect::<Vec<_>>()
			.contains(&slug.to_string())
	}

	fn get_specified_addons(&self, slugs: Vec<String>) -> Result<Vec<(String, String, Vec<u8>)>> {
		let excess: Vec<&String> = slugs.iter().filter(|slug| !self.contains(slug)).collect();

		if !excess.is_empty() {
			return Err(crate::errors::Error::NotInstalled(
				excess
					.into_iter()
					.map(|slug| slug.as_str())
					.collect::<Vec<_>>()
					.join(", "),
			));
		}

		let addons = self
			.addons_json_database
			.addons
			.iter()
			.map(|addon| (addon.slug(), addon.id.clone(), addon.version()))
			.collect::<Vec<_>>();

		Ok(addons)
	}

	fn get_all_addons(&self) -> Vec<(String, String, Vec<u8>)> {
		self.addons_json_database
			.addons
			.iter()
			.map(|addon| (addon.slug(), addon.id.clone(), addon.version()))
			.collect()
	}

	pub fn get_addons(&self, slugs: Option<Vec<String>>) -> Result<Vec<(String, String, Vec<u8>)>> {
		let addons = match slugs {
			Some(slugs) => self.get_specified_addons(slugs)?,
			None => self.get_all_addons(),
		};

		Ok(addons)
	}
}
