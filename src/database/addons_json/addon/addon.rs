use super::contribution_url::ContributionUrl;
use super::creator::Creator;
use super::screenshot::Screenshot;
use crate::addon::addon::Addon;
use crate::api::ADDON_URL;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddonsJsonAddon {
	average_rating: f32,
	#[serde(rename = "contributionURL")]
	contribution_url: ContributionUrl,
	creator: Creator,
	developers: Vec<Creator>,
	full_description: String,
	#[serde(rename = "homepageURL")]
	homepage_url: Url,
	icons: HashMap<String, Url>,
	#[serde(rename = "amoListingURL")]
	amo_listing_url: Url,
	description: String,
	pub id: String,
	name: String,
	version: String,
	review_count: u64,
	#[serde(rename = "reviewURL")]
	review_url: Url,
	screenshots: Vec<Screenshot>,
	#[serde(rename = "sourceURI")]
	source_uri: Url,
	#[serde(rename = "supportURL")]
	support_url: Url,
	#[serde(rename = "type")]
	ty: String,
	update_date: u64,
	weekly_downloads: u64,
}

impl TryFrom<&Addon> for AddonsJsonAddon {
	type Error = Error;

	fn try_from(addon: &Addon) -> Result<Self> {
		let url = Url::parse("https://example.com")?;
		let average_rating = 0.0;
		let amo_listing_url = Url::parse(&format!("{}/{}/", ADDON_URL, &addon.slug))?;
		let contribution_url = ContributionUrl::Empty(String::new());
		let creator = Creator::from(&url);
		let description = String::new();
		let developers = Vec::new();
		let full_description = String::new();
		let homepage_url = url.clone();
		let icons = HashMap::new();
		let id = addon.guid.clone();
		let name = String::new();
		let review_count = 0;
		let review_url = url.clone();
		let screenshots = Vec::new();
		let source_uri = url.clone();
		let support_url = url.clone();
		let ty = String::new();
		let update_date = 0;
		let version = addon.current_version.version.clone();
		let weekly_downloads = 0;

		// TODO:
		let addon = Self {
			amo_listing_url,
			average_rating,
			contribution_url,
			creator,
			description,
			developers,
			full_description,
			homepage_url,
			icons,
			id,
			name,
			review_count,
			review_url,
			screenshots,
			source_uri,
			support_url,
			ty,
			update_date,
			version,
			weekly_downloads,
		};

		Ok(addon)
	}
}

impl AddonsJsonAddon {
	pub fn slug(&self) -> String {
		let mut slug = self
			.amo_listing_url
			.path_segments()
			.map(|segment| segment.collect::<Vec<_>>())
			.unwrap();

		slug.pop().unwrap();
		slug.pop().unwrap().to_string()
	}

	pub fn version(&self) -> Vec<u8> {
		self.version
			.split('.')
			.filter_map(|s| s.parse().ok())
			.collect()
	}
}
