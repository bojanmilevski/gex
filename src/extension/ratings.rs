use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize)]
pub struct Ratings {
	pub average: Option<f32>,
}

impl Display for Ratings {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Rating".bold().bright_blue(), &self.average.unwrap_or(0.0))
	}
}
