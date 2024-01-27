use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Deserialize)]
pub struct Name {
	#[serde(rename = "en-US")]
	pub name: Option<String>,
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Name".bold().bright_blue(), &self.name.clone().unwrap_or("EMPTY".to_owned()))
	}
}
