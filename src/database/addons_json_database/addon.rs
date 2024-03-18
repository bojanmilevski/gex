use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Addon {
	pub description: String,
	pub id: String,
	pub name: String,
	#[serde(rename = "amoListingURL")]
	pub slug: String,
	pub version: String,
}
