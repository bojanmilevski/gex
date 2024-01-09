use super::authors::Authors;
use super::creation_date::CreationDateTime;
use super::current_version::CurrentVersion;
use super::description::Description;
use super::name::Name;
use super::ratings::Ratings;
use super::score::Score;
use super::url::URL;
use super::weekly_downloads::WeeklyDownloads;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Deserialize)]
pub struct Extension {
	pub authors: Authors,
	created: CreationDateTime,
	description: Description,
	pub name: Name,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub slug: String,
	ratings: Ratings,
	#[serde(rename = "_score")]
	score: Score,
	url: URL,
	weekly_downloads: WeeklyDownloads,
}

impl Display for Extension {
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
