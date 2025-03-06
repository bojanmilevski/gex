use super::addons_json::addons_json::AddonsJson;
use super::extensions_json::extensions_json::ExtensionsJson;
use crate::cli::CliConfiguration;
use crate::operation::install::Package;
use crate::profile::Profile;
use anyhow::Result;
use tokio::io::AsyncWriteExt;

pub struct Database {
	profile: Profile,
	addons_json: AddonsJson,
	extensions_json: ExtensionsJson,
	// manifests: Manifests,
}

impl TryFrom<CliConfiguration> for Database {
	type Error = anyhow::Error;

	fn try_from(cli_configuration: CliConfiguration) -> Result<Self> {
		let profile = Profile::try_from(cli_configuration)?;
		let addons_json = AddonsJson::try_from(&profile)?;
		let extensions_json = ExtensionsJson::try_from(&profile)?;
		// let manifests = Manifests::try_from(&extensions_json)?;

		Ok(Self { profile, addons_json, extensions_json /*manifests */ })
	}
}

impl Database {
	pub fn get_slugs(&self) -> Vec<String> {
		self
			.addons_json
			.addons
			.keys()
			.map(|id| id.slug.clone())
			.collect()
	}

	pub fn add(&mut self, addons: &[Package]) -> Result<()> {
		self.addons_json.add(addons)?;
		self.extensions_json.add(addons, &self.profile)?;
		// self.manifests.add(addons)?;

		Ok(())
	}

	pub fn remove_from_database(&mut self, addons: &[Package]) -> Result<()> {
		self.addons_json.remove(&addons)?;
		self.extensions_json.remove(&addons)?;
		// self.manifest_database.remove(&addons)?;

		Ok(())
	}

	pub fn remove_from_disk(&mut self, addons: &[Package]) -> Result<()> {
		// TODO:
		// delete file from profile/extensions/*.xpi fodler

		Ok(())
	}

	pub fn write_to_disk(&self) -> Result<()> {
		self.addons_json.write_to_disk(&self.profile)?;
		self.extensions_json.write_to_disk(&self.profile)?;

		Ok(())
	}

	pub async fn write_new_addons_to_disk<'a>(&self, addons: &[Package<'a>]) -> Result<()> {
		// TODO: futures_util::stream::iter()
		for addon in addons {
			let path = format!(
				"{}.xpi",
				self
					.profile
					.path
					.join("extensions")
					.join(&addon.json_response.guid)
					.display()
			);

			tokio::fs::File::create(path)
				.await?
				.write_all(&addon.xpi)
				.await?;
		}

		Ok(())
	}
}
