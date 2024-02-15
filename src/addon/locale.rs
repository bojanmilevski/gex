use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Locale {
	pub contributors: Option<String>,
	pub creator: Option<String>,
	pub description: Option<String>,
	pub developers: Option<String>,
	pub locales: Option<Vec<String>>,
	pub name: Option<String>,
	pub translators: Option<String>,
}
