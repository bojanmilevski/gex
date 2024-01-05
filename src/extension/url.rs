use std::fmt::Display;

use colored::Colorize;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct URL {
	url: String,
}

impl Display for URL {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "URL".bold().bright_blue(), &self.url)
	}
}
