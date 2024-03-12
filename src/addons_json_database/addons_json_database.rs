use super::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct AddonsJsonDatabase {
	pub addons: Vec<Addon>,
}

impl TryFrom<&Profile> for AddonsJsonDatabase {
	type Error = Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let path = profile.path.join("addons.json");
		let file = std::fs::File::open(path)?;
		let mut reader = BufReader::new(file);
		let mut content = String::new();
		reader.read_to_string(&mut content)?;
		let mut addons: AddonsJsonDatabase = serde_json::from_str(&content).unwrap();

		addons.addons.iter_mut().for_each(|addon| {
			let url = Url::parse(&addon.slug).unwrap();
			let mut segments = url
				.path_segments()
				.map(|segment| segment.collect::<Vec<_>>())
				.unwrap();

			segments.pop().unwrap();
			addon.slug = segments.last().unwrap().to_string();
		});

		Ok(Self {
			addons: addons.addons,
		})
	}
}
