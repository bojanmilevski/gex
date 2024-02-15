use super::addon::Addon;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Addons {
	pub addons: Vec<Addon>,
}

impl TryFrom<&PathBuf> for Addons {
	type Error = Error;

	fn try_from(path: &PathBuf) -> Result<Self> {
		let extensions_json_path = path.join("extensions.json");
		let database = std::fs::File::open(extensions_json_path)?;
		let reader = BufReader::new(database);
		let a_addons: Addons = serde_json::from_reader(reader)?;
		let addons = a_addons
			.addons
			.into_iter()
			.filter(|addon| addon.location != "app-builtin" && addon.location != "app-system-defaults")
			.collect::<Vec<_>>(); // TODO: write deserializer

		Ok(Self { addons })
	}
}
