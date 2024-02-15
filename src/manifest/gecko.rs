use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Gecko {
	pub id: String,
	pub strict_min_version: Option<String>,
	pub strict_max_version: Option<String>,
}
