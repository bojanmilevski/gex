use super::compatibility::Compatibility;
use super::file::File;
use super::license::License;
use chrono::DateTime;
use chrono::Utc;
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

#[derive(Deserialize)]
pub struct CurrentVersion {
	compatibility: Compatibility,
	edit_url: Url,
	id: u64,
	is_strict_compatibility_enabled: bool,
	pub file: File,
	pub license: License,
	pub version: String,
	release_notes: Option<HashMap<String, String>>,
	reviewed: DateTime<Utc>,
}

impl Display for CurrentVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Version".bold().bright_blue(), self.version)
	}
}
