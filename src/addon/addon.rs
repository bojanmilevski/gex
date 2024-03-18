use super::authors::Authors;
use super::creation_date::CreationDateTime;
use super::current_version::CurrentVersion;
use super::description::Description;
use super::name::Name;
use super::ratings::Ratings;
use super::score::Score;
use super::url::Url;
use super::weekly_downloads::WeeklyDownloads;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct Addons {
	#[serde(rename = "results")]
	pub addons: Vec<Addon>,
}

#[derive(Deserialize)]
pub struct Addon {
	authors: Authors,
	created: CreationDateTime,
	description: Description,
	name: Name,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub slug: String,
	ratings: Ratings,
	#[serde(rename = "_score")]
	score: Score,
	url: Url,
	weekly_downloads: WeeklyDownloads,
}

impl Addon {
	pub fn get_name(&self) -> String {
		self.name.name.to_owned().unwrap_or("EMPTY".to_owned())
	}
}

impl Display for Addon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
			self.name,
			self.current_version,
			self.url,
			self.authors,
			self.created,
			self.current_version.license,
			self.ratings,
			self.score,
			self.weekly_downloads,
			self.description,
		)
	}
}
