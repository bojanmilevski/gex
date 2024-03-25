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
	contribution_url: Url,
	creator: Creator,
	developers: Vec<Creator>,
	full_description: String,
	#[serde(rename = "homepageURL")]
	homepage_url: Url,
	icons: HashMap<String, Url>,
	#[serde(rename = "amoListingURL")]
	amo_listing_url: Url,
	description: String,
	id: String,
	name: String,
	version: String,
	review_count: u32,
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
	weekly_downloads: u32,
}

#[derive(Serialize, Deserialize)]
struct Creator {
	name: String,
	url: Url,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Screenshot {
	url: Url,
	width: u32,
	height: u32,
	#[serde(rename = "thumbnailURL")]
	thumbnail_url: Url,
	thumbnail_width: u32,
	thumbnail_height: u32,
	caption: String,
}

impl TryFrom<&Addon> for AddonsJsonAddon {
	type Error = Error;

	fn try_from(addon: &Addon) -> Result<Self> {
		let url = Url::parse("https://crates.io")?;
		let creator = Creator { name: String::new(), url: url.clone() };
		let amo_listing_url = Url::parse(ADDON_URL)?.join("/")?;

		// TODO:
		let addon = Self {
			description: String::new(),
			id: addon.guid.clone(),
			name: addon.get_name(),
			amo_listing_url,
			version: addon.current_version.version.clone(),
			average_rating: 0.0,
			contribution_url: url.clone(),
			creator,
			developers: Vec::new(),
			full_description: String::new(),
			homepage_url: url.clone(),
			icons: HashMap::new(),
			review_count: 0,
			review_url: url.clone(),
			screenshots: Vec::new(),
			source_uri: url.clone(),
			support_url: url.clone(),
			ty: String::new(),
			update_date: 0,
			weekly_downloads: 0,
		};

		Ok(addon)
	}
}

impl AddonsJsonAddon {
	pub fn get_slug(&self) -> Result<String> {
		let segments = self
			.amo_listing_url
			.path_segments()
			.map(|segment| segment.collect::<Vec<_>>())
			.unwrap()
			.pop()
			.unwrap();

		Ok(String::from(segments))
	}
}
