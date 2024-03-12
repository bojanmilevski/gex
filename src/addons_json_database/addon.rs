use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Addon {
	pub id: String,
	pub name: String,
	pub version: String,
	pub description: String,
	#[serde(rename = "amoListingURL")]
	pub slug: String,
}
