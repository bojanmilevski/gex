use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct License {
	pub slug: Option<String>,
}

impl Display for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "License".bold().bright_blue(), &self.slug.to_owned().unwrap_or("EMPTY".to_owned()))
	}
}
