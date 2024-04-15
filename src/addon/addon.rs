use super::authors::Authors;
use super::contributions_url::ContributionsUrl;
use super::current_version::CurrentVersion;
use super::description::Description;
use super::homepage::Homepage;
use super::name::Name;
use super::preview::Preview;
use super::promoted::Promoted;
use super::ratings::Ratings;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

#[derive(Deserialize)]
pub struct Addons {
	count: u64,
	next: Option<Url>,
	page_count: u64,
	page_size: u64,
	previous: Option<Url>,
	#[serde(rename = "results")]
	pub addons: Vec<Addon>,
}

#[derive(Deserialize)]
pub struct Addon {
	authors: Authors,
	average_daily_users: u64,
	categories: Vec<String>,
	contributions_url: ContributionsUrl,
	created: String, // FIX: chrono
	pub default_locale: String,
	description: Description,
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
	last_updated: String, // FIX: chrono
	pub name: Name,
	previews: Vec<Preview>,
	promoted: Option<Promoted>,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub slug: String,
	ratings: Ratings,
	ratings_url: Url,
	requires_payment: bool,
	review_url: Url,
	#[serde(rename = "_score")]
	score: Option<f64>, // FIX:
	status: String,
	summary: Option<HashMap<String, Option<String>>>,
	support_email: Option<HashMap<String, Option<String>>>,
	support_url: Option<Homepage>,
	tags: Vec<String>,
	#[serde(rename = "type")]
	ty: String,
	url: Option<Url>,
	versions_url: Url,
	pub weekly_downloads: u64, // FIX:
}

impl Addon {
	pub fn version(&self) -> Vec<u8> {
		self.current_version
			.version
			.split('.')
			.filter_map(|s| s.parse().ok())
			.collect()
	}
}

impl Display for Addon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
			self.name,
			self.current_version,
			self.url.clone().unwrap(),
			self.authors,
			self.created,
			self.current_version.license,
			self.ratings,
			self.score.unwrap_or(0.0),
			self.weekly_downloads,
			self.description,
		)
	}
}
