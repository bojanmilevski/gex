use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Score {
	score: Option<f64>,
}

impl Display for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {:.2}", "Score".bold().bright_blue(), self.score.unwrap_or(0.0).to_string())
	}
}
