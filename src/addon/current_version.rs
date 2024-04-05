use super::license::License;
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
	reviewed: String,
}

#[derive(Deserialize)]
struct Compatibility {
	firefox: CompatibilityType,
	android: Option<CompatibilityType>,
}

#[derive(Deserialize)]
struct CompatibilityType {
	min: String,
	max: String,
}

#[derive(Deserialize)]
pub struct File {
	pub id: u64,
	created: String, // FIX: should be chrono
	hash: String,    // FIX: should be hash
	is_mozilla_signed_extension: bool,
	size: u64,
	status: String,
	url: Url,
	permissions: Vec<String>,
	optional_permissions: Vec<String>,
	host_permissions: Vec<String>,
}

impl Display for CurrentVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Version".bold().bright_blue(), self.version)
	}
}
