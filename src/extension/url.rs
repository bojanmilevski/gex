use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(transparent, rename = "URL")]
pub struct Url {
	url: String,
}

impl Display for Url {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "URL".bold().bright_blue(), self.url)
	}
}
