use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct WeeklyDownloads {
	weekly_downloads: i32,
}

impl Display for WeeklyDownloads {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", "Weekly downloads".bold().bright_blue(), self.weekly_downloads)
	}
}
