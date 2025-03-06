#![allow(dead_code)]
#![allow(clippy::large_enum_variant)]

use chrono::DateTime;
use chrono::Utc;
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
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
	pub authors: Authors,
	average_daily_users: u64,
	categories: Vec<String>,
	pub contributions_url: ContributionsUrl, // FIX: change struct name
	created: DateTime<Utc>,
	default_locale: String,
	pub description: Description,
	developer_comments: Option<HashMap<String, String>>,
	edit_url: Url,
	has_eula: bool,
	has_privacy_policy: bool,
	pub homepage: Option<Homepage>, // FIX: change struct name
	icon_url: Url,
	icons: HashMap<String, Url>,
	id: u64,
	is_disabled: bool,
	is_experimental: bool,
	pub last_updated: DateTime<Utc>,
	pub previews: Vec<Preview>,
	promoted: Option<Promoted>,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub name: Name,
	pub slug: String, // FIX: should be option
	pub ratings: Ratings,
	pub ratings_url: Url,
	requires_payment: bool,
	review_url: Url,
	#[serde(rename = "_score")]
	score: Option<f64>, // FIX: should not be option
	status: String,
	summary: Option<HashMap<String, Option<String>>>,
	support_email: Option<HashMap<String, Option<String>>>,
	pub support_url: Option<Homepage>, // FIX: change struct name
	tags: Vec<String>,
	#[serde(rename = "type")]
	pub ty: String,
	pub url: Option<Url>,
	versions_url: Url,
	pub weekly_downloads: u64, // FIX: is always 0 for some reason
}

impl Addon {
	pub fn version(&self) -> Vec<u8> {
		self
			.current_version
			.version
			.split('.')
			.filter_map(|s| s.parse().ok())
			.collect()
	}
}

impl Display for Addon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n{}\n{}\n", self.name, self.current_version, self.url.as_ref().unwrap(),)
	}
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Authors {
	pub authors: Vec<Author>,
}

#[derive(Deserialize)]
pub struct Author {
	pub name: String,
	id: u64,
	pub url: Url,
	username: String,
	picture_url: Option<Url>,
}

impl Authors {
	fn get_joined(&self) -> String {
		self
			.authors
			.iter()
			.map(|author| String::from(&author.name))
			.collect::<Vec<_>>()
			.join(", ")
	}
}

impl Display for Authors {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Authors".bold().bright_blue(), self.get_joined())
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ContributionsUrl {
	UrlOutgoing(UrlOutgoing),
	Empty(String), // is always an empty string
}

#[derive(Deserialize)]
pub struct UrlOutgoing {
	pub url: Url,
	outgoing: Url,
}

#[derive(Deserialize)]
pub struct Compatibility {
	firefox: CompatibilityType,
	android: Option<CompatibilityType>,
}

#[derive(Deserialize)]
pub struct CompatibilityType {
	min: String,
	max: String,
}

#[derive(Deserialize)]
pub struct CurrentVersion {
	compatibility: Compatibility,
	edit_url: Url,
	id: u64,
	is_strict_compatibility_enabled: bool,
	pub file: File,
	license: License,
	pub version: String,
	release_notes: Option<HashMap<String, String>>,
	reviewed: DateTime<Utc>,
}

impl Display for CurrentVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Version".bold().bright_blue(), self.version)
	}
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Description {
	pub description: Option<HashMap<String, Option<String>>>,
}

impl Display for Description {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.description
				.clone()
				.unwrap()
				.get("en-US")
				.unwrap()
				.clone()
				.unwrap_or(String::from("None"))
		)
	}
}

#[derive(Deserialize)]
pub struct File {
	id: u64,
	created: DateTime<Utc>,
	hash: String,
	is_mozilla_signed_extension: bool,
	size: u64,
	status: String,
	pub url: Url,
	pub permissions: Vec<String>,
	optional_permissions: Vec<String>,
	host_permissions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Homepage {
	pub url: HashMap<String, Option<Url>>,
	outgoing: HashMap<String, Option<Url>>,
}

#[derive(Deserialize)]
pub struct License {
	slug: Option<String>,
	id: u64,
	is_custom: bool,
	name: HashMap<String, String>,
	url: Option<Url>,
}

impl Display for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "License".bold().bright_blue(), &self.slug.clone().unwrap_or("EMPTY".to_owned()))
	}
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Name {
	pub name: HashMap<String, Option<String>>,
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.name
				.clone()
				.get("en-US")
				.unwrap()
				.clone()
				.unwrap_or(String::from("None"))
		)
	}
}

#[derive(Deserialize)]
pub struct Preview {
	id: u64,
	pub caption: Option<HashMap<String, String>>,
	pub image_size: [u64; 2],
	pub image_url: Url,
	position: u64,
	pub thumbnail_size: Option<[u64; 2]>,
	pub thumbnail_url: Url,
}

#[derive(Deserialize)]
pub struct Promoted {
	apps: Vec<String>,
	category: String,
}

#[derive(Deserialize)]
pub struct Ratings {
	pub average: Option<f32>,
	bayesian_average: f64,
	count: u64,
	pub text_count: u64,
}

impl Display for Ratings {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Average rating".bold().bright_blue(), &self.average.unwrap_or(0.0).to_owned())
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Response {
	Addon(Addon),
	NotFound(NotFound),
	Authentication(Authentication),
}

#[derive(Deserialize)]
pub struct NotFound {
	detail: String,
}

#[derive(Deserialize)]
pub struct Authentication {
	detail: String,
	is_disabled_by_developer: bool,
	is_disabled_by_mozilla: bool,
}
