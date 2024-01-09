use super::license::License;
use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Deserialize)]
pub struct CurrentVersion {
	pub file: File,
	pub license: License,
	pub version: String,
}

#[derive(Clone, Deserialize)]
pub struct File {
	pub id: i32,
}

impl Display for CurrentVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Version".bold().bright_blue(), self.version)
	}
}
