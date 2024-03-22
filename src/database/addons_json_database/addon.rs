use crate::addon::addon::Addon as MainAddon;
use crate::api::ADDON_URL;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddonsJsonAddon {
	pub description: String,
	pub id: String,
	pub name: String,
	#[serde(rename = "amoListingURL")]
	pub slug: String,
	pub version: String,
}

impl TryFrom<&MainAddon> for AddonsJsonAddon {
	type Error = Error;

	fn try_from(main_addon: &MainAddon) -> Result<Self> {
		let addon = Self {
			description: String::new(), // TODO:
			id: main_addon.guid.clone(),
			name: main_addon.get_name(),
			slug: format!("{}/{}/", ADDON_URL, main_addon.slug.clone()),
			version: main_addon.current_version.version.clone(),
		};

		Ok(addon)
	}
}

impl AddonsJsonAddon {
	pub fn get_slug(&self) -> Result<String> {
		let url = url::Url::parse(&self.slug)?;
		let mut segments = url
			.path_segments()
			.map(|segment| segment.collect::<Vec<_>>())
			.unwrap();

		segments.pop().unwrap();

		Ok(String::from(*segments.last().unwrap()))
	}
}
