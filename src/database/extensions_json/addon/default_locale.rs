use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct DefaultLocale {
	pub contributors: Option<String>,
	pub creator: Option<String>,
	pub description: Option<String>,
	pub developers: Option<String>,
	#[serde(rename = "homepageURL")]
	pub homepage_url: Option<Url>,
	pub name: Option<String>,
	pub translators: Option<String>,
}
