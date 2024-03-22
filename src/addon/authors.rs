use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Authors {
	authors: Vec<Author>,
}

#[derive(Deserialize)]
pub struct Author {
	name: String,
}

impl Authors {
	pub fn get_joined(&self) -> String {
		self.authors
			.iter()
			.map(|author| String::from(&author.name))
			.collect::<Vec<_>>()
			.join(", ")
	}
}

impl Display for Authors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Authors".bold().bright_blue(), self.get_joined())
	}
}
