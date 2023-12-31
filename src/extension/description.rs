use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct Description {
	description: Option<Language>,
}

#[derive(Clone, Deserialize)]
pub struct Language {
	#[serde(rename = "en-US")]
	language: Option<String>,
}

impl Display for Description {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {}",
			"Description".bold().bright_blue(),
			self.description
				.clone()
				.unwrap_or(Language { language: None })
				.language
				.unwrap_or("None".to_owned())
				.replace("\n", " ")
		)
	}
}
