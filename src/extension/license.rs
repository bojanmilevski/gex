use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct License {
	slug: Option<String>,
}

impl Display for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "License".bold().bright_blue(), &self.slug.clone().unwrap_or("EMPTY".to_owned()))
	}
}
