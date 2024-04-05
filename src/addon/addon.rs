use super::authors::Authors;
use super::current_version::CurrentVersion;
use super::ratings::Ratings;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

#[derive(Deserialize)]
pub struct Addons {
	#[serde(rename = "results")]
	pub addons: Vec<Addon>,
}

#[derive(Deserialize)]
pub struct Addon {
	authors: Authors, // FIX:
	average_daily_users: u64,
	categories: Vec<String>,
	// contributions_url: HashMap<String, Url>, // FIX:
	created: String, // FIX: should be chrono
	default_locale: String,
	description: Option<HashMap<String, Option<String>>>,
	developer_comments: Option<HashMap<String, String>>,
	edit_url: Url,
	has_eula: bool,
	has_privacy_policy: bool,
	homepage: Option<Homepage>,
	icon_url: Url,
	icons: HashMap<String, Url>,
	id: u64,
	is_disabled: bool,
	is_experimental: bool,
	last_updated: String, // FIX: should be chrono
	name: HashMap<String, Option<String>>,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub slug: Option<String>, // FIX: ???
	ratings: Ratings,
	#[serde(rename = "_score")]
	score: Option<f64>, // FIX:
	weekly_downloads: u64, // FIX:
	previews: Vec<Preview>,
	promoted: Option<Promoted>,
	ratings_url: Url,
	requires_payment: bool,
	review_url: Url,
	status: String,
	summary: Option<HashMap<String, Option<String>>>,
	support_email: Option<HashMap<String, Option<String>>>,
	support_url: Option<Homepage>,
	tags: Vec<String>,
	url: Option<Url>,
	versions_url: Url,
}

#[derive(Deserialize)]
struct Homepage {
	url: HashMap<String, Option<String>>, // FIX: value should be url
	outgoing: HashMap<String, Option<String>>,
}

#[derive(Deserialize)]
struct Preview {
	id: u64,
	caption: Option<HashMap<String, String>>,
	image_size: [u64; 2],
	image_url: Url,
	position: u64,
	tuhmbnail_size: Option<[u64; 2]>,
	thumbnail_url: Url,
}

#[derive(Deserialize)]
struct Promoted {
	apps: Vec<String>,
	category: String,
}

impl Addon {
	pub fn get_name(&self) -> String {
		self.name.clone().get("en-US").unwrap().clone().unwrap()
	}
}

impl Display for Addon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
			self.get_name(),
			self.current_version,
			self.url.clone().unwrap(),
			self.authors,
			self.created,
			self.current_version.license,
			self.ratings,
			self.score.unwrap_or(0.0),
			self.weekly_downloads,
			self.description
				.clone()
				.unwrap_or(Default::default())
				.get("en-US")
				.unwrap()
				.clone()
				.unwrap(),
		)
	}
}
