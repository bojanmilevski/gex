use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct Ratings {
	average: Option<f32>,
	bayesian_average: f64,
	count: u64,
	text_count: u64,
}

impl Display for Ratings {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Average rating".bold().bright_blue(), &self.average.unwrap_or(0.0).to_owned())
	}
}
