use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct Name {
	#[serde(rename = "en-US")]
	pub name: Option<String>,
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Name".bold().bright_blue(), self.name.to_owned().unwrap_or("EMPTY".to_owned()))
	}
}
