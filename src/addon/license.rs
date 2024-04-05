use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

#[derive(Deserialize)]
pub struct License {
	slug: Option<String>,
	id: u64,
	is_custom: bool,
	name: HashMap<String, String>,
	url: Option<Url>,
}

impl Display for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "License".bold().bright_blue(), &self.slug.clone().unwrap_or("EMPTY".to_owned()))
	}
}
