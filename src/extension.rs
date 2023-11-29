use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResult {
	pub results: Vec<Extension>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Extension {
	pub id: i32,
	pub authors: Vec<Author>,
	pub categories: Vec<String>,
	pub created: String,
	pub description: Option<Description>,
	pub current_version: CurrentVersion,
	pub guid: String,
	pub name: Name,
	pub ratings: Ratings,
	pub slug: String,
	pub url: String,
	pub weekly_downloads: i32,
	#[serde(rename = "_score")]
	pub score: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
	pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Description {
	#[serde(rename = "en-US")]
	pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentVersion {
	pub file: FileCurrentVersion,
	pub license: License,
	pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileCurrentVersion {
	pub id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct License {
	pub slug: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ratings {
	pub average: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Name {
	#[serde(rename = "en-US")]
	pub name: Option<String>,
}

impl Extension {
	fn get_authors_as_string(&self) -> String {
		self.authors
			.iter()
			.map(|author| author.name.clone())
			.collect::<Vec<String>>()
			.join(", ")
	}
}

impl Ord for Extension {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.score.partial_cmp(&other.score).unwrap()
	}
}

impl PartialOrd for Extension {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.score.partial_cmp(&other.score)
	}
}

impl Eq for Extension {}

impl PartialEq for Extension {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
			&& self.authors == other.authors
			&& self.categories == other.categories
			&& self.created == other.created
			&& self.description == other.description
			&& self.current_version == other.current_version
			&& self.guid == other.guid
			&& self.name == other.name
			&& self.ratings == other.ratings
			&& self.slug == other.slug
			&& self.url == other.url
			&& self.weekly_downloads == other.weekly_downloads
			&& self.score == other.score
	}
}

impl Eq for Author {}

impl PartialEq for Author {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Description {}

impl PartialEq for Description {
	fn eq(&self, other: &Self) -> bool {
		self.description == other.description
	}
}

impl Eq for CurrentVersion {}

impl PartialEq for CurrentVersion {
	fn eq(&self, other: &Self) -> bool {
		self.file == other.file && self.license == other.license && self.version == other.version
	}
}

impl Eq for FileCurrentVersion {}

impl PartialEq for FileCurrentVersion {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for License {}

impl PartialEq for License {
	fn eq(&self, other: &Self) -> bool {
		self.slug == other.slug
	}
}

impl Eq for Name {}

impl PartialEq for Name {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Ratings {}

impl PartialEq for Ratings {
	fn eq(&self, other: &Self) -> bool {
		self.average == other.average
	}
}

impl Display for Extension {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
			"Name".bold().bright_blue(),
			&self.name.name.clone().unwrap_or("EMPTY".to_string()),
			"Version".bold().bright_blue(),
			&self.current_version.version,
			"URL".bold().bright_blue(),
			&self.url,
			"Authors".bold().bright_blue(),
			&self.get_authors_as_string(),
			"Created".bold().bright_blue(),
			&self.created,
			"License".bold().bright_blue(),
			&self
				.current_version
				.license
				.clone()
				.slug
				.unwrap_or("EMPTY".to_owned()),
			"Rating".bold().bright_blue(),
			&self.ratings.average,
			"Score".bold().bright_blue(),
			&self.score,
			"Weekly downloads".bold().bright_blue(),
			&self.weekly_downloads,
			"Description".bold().bright_blue(),
			&self
				.description
				.clone()
				.unwrap_or(Description { description: None })
				.description
				.unwrap_or("EMPTY".to_owned()),
		)
	}
}
