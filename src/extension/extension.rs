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

#[derive(Debug, Clone, Deserialize)]
pub struct Extension {
	// id: i32,
	authors: Authors,
	// pub categories: Vec<String>,
	created: CreationDateTime,
	description: Description,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub name: Name,
	ratings: Ratings,
	pub slug: String,
	url: URL,
	weekly_downloads: WeeklyDownloads,
	#[serde(rename = "_score")]
	score: Score,
}

impl Into<String> for Extension {
	fn into(self) -> String {
		self.name.into()
	}
}

impl Display for Extension {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
			&self.name,
			&self.current_version,
			&self.url,
			&self.authors,
			&self.created,
			&self.current_version.license,
			&self.ratings,
			&self.score,
			&self.weekly_downloads,
			&self.description,
		)
	}
}
