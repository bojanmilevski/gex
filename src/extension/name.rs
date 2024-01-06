use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct Name {
	#[serde(rename = "en-US")]
	name: Option<String>,
}

impl Into<String> for Name {
	fn into(self) -> String {
		self.name.unwrap_or("EMPTY".to_string())
	}
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Name".bold().bright_blue(), &self.name.to_owned().unwrap_or("EMPTY".to_string()))
	}
}
