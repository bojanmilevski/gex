use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionsList {
	#[serde(rename = "results")]
	pub extensions: Vec<Extension>,
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
	pub score: Option<f32>,
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
	pub average: Option<f32>,
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

impl Display for Extension {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
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
			&self.ratings.average.unwrap_or(0.0),
			"Score".bold().bright_blue(),
			&self.score.unwrap_or(0.0),
			"Weekly downloads".bold().bright_blue(),
			&self.weekly_downloads,
			/* "Description".bold().bright_blue(),
			&self
				.description
				.clone()
				.unwrap_or(Description { description: None })
				.description
				.unwrap_or("EMPTY".to_owned()), */
		)
	}
}
