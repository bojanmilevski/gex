use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
	pub manifest_version: u8,
	pub name: String,
	pub version: String,
	pub author: String,
	pub description: String,
	pub default_locale: String,
	pub icons: HashMap<String, String>,
	pub permissions: Vec<String>,
	pub optional_permissions: Option<Vec<String>>,
	pub browser_specific_settings: BrowserSpecificSettings,
}

#[derive(Serialize, Deserialize)]
pub struct BrowserSpecificSettings {
	pub gecko: Gecko,
}

#[derive(Serialize, Deserialize)]
pub struct Gecko {
	pub id: String,
	pub strict_min_version: Option<String>,
	pub strict_max_version: Option<String>,
}
