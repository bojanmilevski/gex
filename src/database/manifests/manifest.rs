use super::browser_specific_settings::BrowserSpecificSettings;
use crate::database::extensions_json::addon::addon::ExtensionsJsonAddon;
use anyhow::Error;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use zip::ZipArchive;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
	pub author: Option<String>,
	pub browser_specific_settings: Option<BrowserSpecificSettings>,
	pub default_locale: Option<String>,
	pub description: Option<String>,
	pub icons: HashMap<String, String>,
	pub manifest_version: u8,
	pub name: String,
	pub optional_permissions: Option<Vec<String>>,
	pub permissions: Option<Vec<String>>,
	pub version: String,
	pub applications: Option<Applications>,
}

#[derive(Serialize, Deserialize)]
pub struct Applications {
	gecko: Gecko,
}

#[derive(Serialize, Deserialize)]
pub struct Gecko {
	id: String,
}

// FIX: redundant duplication
impl TryFrom<&ExtensionsJsonAddon> for Manifest {
	type Error = Error;

	fn try_from(addon: &ExtensionsJsonAddon) -> Result<Self> {
		let file = std::fs::File::open(addon.path.as_ref().unwrap())?;
		let mut zip = ZipArchive::new(file).unwrap();
		let mut manifest_file = zip.by_name("manifest.json").unwrap();
		let mut content = String::new();
		manifest_file.read_to_string(&mut content).unwrap();
		let manifest: Manifest = serde_json::from_str(content.as_str())?;

		Ok(manifest)
	}
}

// FIX: redundant duplication
impl TryFrom<&Vec<u8>> for Manifest {
	type Error = Error;

	fn try_from(bytes: &Vec<u8>) -> Result<Self> {
		let cursor = Cursor::new(bytes);
		let mut zip = ZipArchive::new(cursor).unwrap();
		let mut manifest_file = zip.by_name("manifest.json").unwrap();
		let mut content = String::new();
		manifest_file.read_to_string(&mut content).unwrap();
		let manifest: Manifest = serde_json::from_str(content.as_str())?;

		Ok(manifest)
	}
}
