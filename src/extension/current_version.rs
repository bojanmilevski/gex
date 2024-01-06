use super::file::File;
use super::license::License;
use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentVersion {
	pub file: File,
	pub license: License,
	version: String,
}

impl Display for CurrentVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Version".bold().bright_blue(), &self.version)
	}
}
