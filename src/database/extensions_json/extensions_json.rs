use std::collections::HashMap;

use super::addon::ExtensionsJsonAddon;
use crate::database::identifier::Identifier;
use crate::operation::install::Package;
use crate::profile::Profile;
use anyhow::Result;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsJson {
	schema_version: u8,

	#[serde(deserialize_with = "deserialize_to_hashmap", serialize_with = "serialize_to_vec")]
	pub addons: HashMap<Identifier, ExtensionsJsonAddon>,
}

fn deserialize_to_hashmap<'de, D>(
	deserializer: D,
) -> std::result::Result<HashMap<Identifier, ExtensionsJsonAddon>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	struct ExtensionsJsonVisitor;

	impl<'de> Visitor<'de> for ExtensionsJsonVisitor {
		type Value = HashMap<Identifier, ExtensionsJsonAddon>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a hashmap of identifier keys and extensions.json addon values")
		}

		fn visit_seq<A>(self, mut seq: A) -> std::result::Result<HashMap<Identifier, ExtensionsJsonAddon>, A::Error>
		where
			A: SeqAccess<'de>,
		{
			let mut map = HashMap::new();

			while let Some(item) = seq.next_element::<ExtensionsJsonAddon>()? {
				if map
					.insert(Identifier { id: item.id.clone(), slug: item.slug().clone() }, item)
					.is_some()
				{
					return Err(serde::de::Error::duplicate_field("duplicate ids found"));
				}
			}

			Ok(map)
		}
	}

	deserializer.deserialize_seq(ExtensionsJsonVisitor)
}

fn serialize_to_vec<S>(
	hashmap: &HashMap<Identifier, ExtensionsJsonAddon>,
	serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let vec: Vec<&ExtensionsJsonAddon> = hashmap.values().collect();
	vec.serialize(serializer)
}

impl TryFrom<&Profile> for ExtensionsJson {
	type Error = anyhow::Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let content = std::fs::read_to_string(profile.path.join("extensions.json"))?;

		// FIX: DEBUG INFO
		let deserialized = &mut serde_json::Deserializer::from_str(&content);
		let res: Result<Self, _> = serde_path_to_error::deserialize(deserialized);

		match res {
			Ok(_) => (),
			Err(e) => println!("Failed to parse extensions.json: {}", e.path()),
		}
		// FIX: DEBUG INFO

		let addons = serde_json::from_str(&content)?;

		Ok(addons)
	}
}

impl ExtensionsJson {
	pub fn add(&mut self, addons: &[Package], profile: &Profile) -> Result<()> {
		self.addons.extend(
			addons
				.iter()
				.map(|addon| ExtensionsJsonAddon::try_from((addon, profile)))
				.collect::<Result<Vec<ExtensionsJsonAddon>>>()?,
		);

		Ok(())
	}

	pub fn remove(&mut self, addons: &[Package]) -> Result<()> {
		// TODO:

		Ok(())
	}

	pub fn write_to_disk(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("extensions.json"), content)?;

		Ok(())
	}
}
