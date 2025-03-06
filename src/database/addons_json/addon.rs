use crate::addon::Addon;
use crate::addon::ContributionsUrl;
use crate::addon::Preview;
use crate::operation::api::ADDON_URL;
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
	full_description: Option<String>,
	#[serde(rename = "homepageURL")]
	homepage_url: Option<Url>,
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
	screenshots: Screenshots,
	#[serde(rename = "sourceURI")]
	source_uri: Url,
	#[serde(rename = "supportURL")]
	support_url: Option<Url>,
	#[serde(rename = "type")]
	ty: String,
	update_date: u64,
	weekly_downloads: u64,
}

// FIX: this mess
impl From<&Addon> for AddonsJsonAddon {
	fn from(addon: &Addon) -> Self {
		let average_rating = addon.ratings.average.unwrap();
		let amo_listing_url =
			Url::parse(&format!("{}/{}/", ADDON_URL, addon.slug)).expect("Error creating amo_listing_url");
		let contribution_url = ContributionUrl::from(addon);
		let creator = Creator::from(addon);
		let description = addon
			.description
			.description
			.as_ref()
			.unwrap()
			.get("en-US")
			.unwrap()
			.clone()
			.unwrap();
		let developers = Vec::new();
		let full_description = Some(description.clone());
		let homepage_url = addon
			.homepage
			.as_ref()
			.unwrap()
			.url
			.get("en-US")
			.unwrap()
			.clone();
		let icons = HashMap::new();
		let id = addon.guid.clone();
		let name = addon
			.name
			.name
			.clone()
			.get("en-US")
			.unwrap()
			.clone()
			.unwrap();
		let review_count = addon.ratings.text_count;
		let review_url = addon.ratings_url.clone();
		let screenshots = Screenshots::from(addon);
		let source_uri = addon.current_version.file.url.clone();
		let support_url = addon
			.support_url
			.as_ref()
			.unwrap()
			.url
			.get("en-US")
			.unwrap()
			.to_owned();
		let ty = addon.ty.clone();
		let update_date = addon.last_updated.timestamp_millis() as u64;
		let version = addon.current_version.version.clone();
		let weekly_downloads = addon.weekly_downloads;

		Self {
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
		}
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
		self
			.version
			.split('.')
			.filter_map(|s| s.parse().ok())
			.collect()
	}
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContributionUrl {
	Url(Url),
	Empty(String), // is always an empty string
}

impl From<&Addon> for ContributionUrl {
	fn from(addon: &Addon) -> Self {
		match &addon.contributions_url {
			ContributionsUrl::UrlOutgoing(val) => Self::Url(val.url.clone()),
			ContributionsUrl::Empty(_) => Self::Empty(String::new()),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Creator {
	name: String,
	url: Url,
}

impl From<&Addon> for Creator {
	fn from(addon: &Addon) -> Self {
		let author = addon.authors.authors.first().expect("No author found");
		let name = author.name.clone();
		let url = author.url.clone();

		Self { name, url }
	}
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Screenshots {
	screenshots: Vec<Screenshot>,
}

impl From<&Addon> for Screenshots {
	fn from(addon: &Addon) -> Self {
		let screenshots = addon
			.previews
			.iter()
			.map(Screenshot::from)
			.collect::<Vec<Screenshot>>();

		Self { screenshots }
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot {
	url: Url,
	width: u64,
	height: u64,
	#[serde(rename = "thumbnailURL")]
	thumbnail_url: Url,
	thumbnail_width: u64,
	thumbnail_height: u64,
	caption: Option<String>,
}

impl From<&Preview> for Screenshot {
	fn from(preview: &Preview) -> Self {
		Self {
			url: preview.image_url.clone(),
			width: preview.image_size[0],
			height: preview.image_size[1],
			thumbnail_url: preview.thumbnail_url.clone(),
			thumbnail_width: preview.thumbnail_size.unwrap().first().unwrap().clone(),
			thumbnail_height: preview.thumbnail_size.unwrap().get(1).unwrap().clone(),
			caption: Some(
				preview
					.caption
					.clone()
					.unwrap()
					.get("en-US")
					.unwrap()
					.to_string(),
			),
		}
	}
}
