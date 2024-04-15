use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Description {
	description: Option<HashMap<String, Option<String>>>,
}

impl Display for Description {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.description
				.clone()
				.unwrap()
				.get("en-US")
				.unwrap()
				.clone()
				.unwrap_or(String::from("None"))
		)
	}
}
