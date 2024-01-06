use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Authors {
	authors: Vec<Author>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
	name: String,
}

impl Authors {
	fn get_joined(&self) -> String {
		self.authors
			.iter()
			.map(|author| author.name.to_owned())
			.collect::<Vec<String>>()
			.join(", ")
	}
}

impl Display for Authors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Authors".bold().bright_blue(), &self.get_joined())
	}
}
