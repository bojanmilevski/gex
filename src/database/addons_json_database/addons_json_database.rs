use super::addon::Addon as SelfAddon;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::traits::crud::CRUD;
use crate::traits::deserializeable::Deserializable;
use crate::traits::serializeable::Serializable;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct AddonsJsonDatabase {
	pub addons: Vec<SelfAddon>,
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

		// TODO: deserializer
		addons.addons.iter_mut().for_each(|addon| {
			let url = Url::parse(&addon.slug).unwrap();
			let mut segments = url
				.path_segments()
				.map(|segment| segment.collect::<Vec<_>>())
				.unwrap();

			segments.pop().unwrap();
			addon.slug = segments.last().unwrap().to_string();
		});

		Ok(Self { addons: addons.addons })
	}
}

impl Serializable for AddonsJsonDatabase {
	fn serialize(&self) -> Result<()> {
		todo!()
	}
}

impl Deserializable for AddonsJsonDatabase {
	fn deserialize(&self) -> Result<()> {
		todo!()
	}
}

impl CRUD for AddonsJsonDatabase {
	fn create() -> Result<()> {
		todo!()
	}

	fn read(&self) -> Result<()> {
		todo!()
	}

	fn update(&self, _addon: Addon) -> Result<()> {
		todo!()
	}

	fn delete(&self, _addon: Addon) -> Result<()> {
		todo!()
	}
}
