use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Name {
	name: HashMap<String, Option<String>>,
}

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.name
				.clone()
				.get("en-US")
				.unwrap()
				.clone()
				.unwrap_or(String::from("None"))
		)
	}
}
