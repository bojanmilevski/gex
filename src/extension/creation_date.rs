use chrono::Datelike;
use chrono::Timelike;
use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct CreationDateTime {
	creation_date: String,
}

impl Display for CreationDateTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let parsed_datetime = chrono::DateTime::parse_from_rfc3339(&*self.creation_date)
			.map_err(|_| return std::fmt::Error)
			.unwrap();

		write!(
			f,
			"{}: {:02}.{:02}.{:04} {:02}:{:02}:{:02}",
			"Created".bold().bright_blue(),
			parsed_datetime.day(),
			parsed_datetime.month(),
			parsed_datetime.year(),
			parsed_datetime.hour(),
			parsed_datetime.minute(),
			parsed_datetime.second()
		)
	}
}
