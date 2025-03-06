use super::super::extensions_json::addon::ExtensionsJsonAddon;
use anyhow::Error;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use zip::ZipArchive;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
	pub author: Option<String>,
	pub browser_specific_settings: Option<BrowserSpecificSettings>,
	pub default_locale: Option<String>,
	pub description: Option<String>,
	pub icons: Option<HashMap<String, String>>,
	pub manifest_version: u8,
	pub name: String,
	pub optional_permissions: Option<Vec<String>>,
	pub permissions: Option<Vec<String>>,
	pub version: String,
	pub applications: Option<Applications>,
}

impl Manifest {
	fn parse_content<C: Read + Seek>(content: C) -> Result<Self> {
		let mut zip = ZipArchive::new(content).unwrap();
		let manifest_file = zip.by_name("manifest.json").unwrap();
		let manifest: Manifest = serde_json::from_reader(manifest_file)?;

		// FIX: DEBUG INFO
		let manifest_file = zip.by_name("manifest.json").unwrap();
		let deserialized = &mut serde_json::Deserializer::from_reader(manifest_file);
		let res: Result<Self, _> = serde_path_to_error::deserialize(deserialized);

		match res {
			Ok(_) => (),
			Err(e) => println!("Failed to parse manifest: {}", e.path()),
		}
		// FIX: DEBUG INFO

		Ok(manifest)
	}
}

impl TryFrom<&ExtensionsJsonAddon> for Manifest {
	type Error = Error;

	fn try_from(addon: &ExtensionsJsonAddon) -> Result<Self> {
		let file = std::fs::File::open(addon.path.as_ref().unwrap())?;
		let manifest = Self::parse_content(file)?;

		Ok(manifest)
	}
}

impl TryFrom<&Vec<u8>> for Manifest {
	type Error = Error;

	fn try_from(bytes: &Vec<u8>) -> Result<Self> {
		let cursor = Cursor::new(bytes);
		let manifest = Self::parse_content(cursor)?;

		Ok(manifest)
	}
}

#[derive(Serialize, Deserialize)]
pub struct Applications {
	pub gecko: Gecko,
}

#[derive(Serialize, Deserialize)]
pub struct BrowserSpecificSettings {
	pub gecko: Gecko,
}

#[derive(Serialize, Deserialize)]
pub struct Gecko {
	pub id: String,
	pub strict_max_version: Option<String>,
	pub strict_min_version: Option<String>,
}
