use super::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
	pub addons: Vec<Addon>,
}

impl TryFrom<&Profile> for Database {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let extensions_json_path = profile.path.join("extensions.json");
		let database = std::fs::File::open(extensions_json_path)?;
		let reader = BufReader::new(database);
		let mut addons: Database = serde_json::from_reader(reader)?;

		addons.addons.retain(|addon| {
			addon.location != "app-builtin" && addon.location != "app-system-defaults"
		});

		Ok(Self {
			addons: addons.addons,
		})
	}
}
