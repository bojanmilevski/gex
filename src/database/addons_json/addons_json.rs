use super::addon::AddonsJsonAddon;
use crate::database::identifier::Identifier;
use crate::operation::install::Package;
use crate::profile::Profile;
use anyhow::Result;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AddonsJson {
	schema: u8,

	#[serde(deserialize_with = "deserialize_to_hashmap", serialize_with = "serialize_to_vec")]
	pub addons: HashMap<Identifier, AddonsJsonAddon>,
}

fn deserialize_to_hashmap<'de, D>(
	deserializer: D,
) -> std::result::Result<HashMap<Identifier, AddonsJsonAddon>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	struct AddonsJsonVisitor;

	impl<'de> Visitor<'de> for AddonsJsonVisitor {
		type Value = HashMap<Identifier, AddonsJsonAddon>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a hashmap of identifier keys and addons.json addon values")
		}

		fn visit_seq<A>(self, mut seq: A) -> std::result::Result<HashMap<Identifier, AddonsJsonAddon>, A::Error>
		where
			A: SeqAccess<'de>,
		{
			let mut map = HashMap::new();

			while let Some(item) = seq.next_element::<AddonsJsonAddon>()? {
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

	deserializer.deserialize_seq(AddonsJsonVisitor)
}

fn serialize_to_vec<S>(
	hashmap: &HashMap<Identifier, AddonsJsonAddon>,
	serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let vec: Vec<&AddonsJsonAddon> = hashmap.values().collect();
	vec.serialize(serializer)
}

impl TryFrom<&Profile> for AddonsJson {
	type Error = anyhow::Error;

	fn try_from(profile: &Profile) -> Result<Self> {
		let content = std::fs::read_to_string(profile.path.join("addons.json"))?;

		// FIX: DEBUG INFO
		let deserialized = &mut serde_json::Deserializer::from_str(&content);
		let res: Result<Self, _> = serde_path_to_error::deserialize(deserialized);
		match res {
			Ok(_) => (),
			Err(e) => println!("Failed to parse addons.json: {}", e.path()),
		}
		// FIX: DEBUG INFO

		let addons: AddonsJson = serde_json::from_str(&content)?;

		Ok(addons)
	}
}

impl AddonsJson {
	pub fn add(&mut self, addons: &[Package]) -> Result<()> {
		self.addons.extend(
			addons
				.iter()
				.map(|addon| {
					(
						Identifier { id: addon.json_response.guid.clone(), slug: addon.json_response.slug.clone() },
						AddonsJsonAddon::from(addon.json_response),
					)
				})
				.collect::<HashMap<Identifier, AddonsJsonAddon>>(),
		);

		Ok(())
	}

	pub fn remove(&mut self, addons: &[Package]) -> Result<()> {
		// TODO:

		Ok(())
	}

	pub fn write_to_disk(&self, profile: &Profile) -> Result<()> {
		let content = serde_json::to_string(&self)?;
		std::fs::write(profile.path.join("addons.json"), content)?;

		Ok(())
	}
}
