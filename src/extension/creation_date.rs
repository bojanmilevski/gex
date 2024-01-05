use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct CreationDate {
	creation_date: String,
}

impl Display for CreationDate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Score".bold().bright_blue(), &self.creation_date)
	}
}
