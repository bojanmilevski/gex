use chrono::Datelike;
use chrono::Timelike;
use colored::Colorize;
use serde::Deserialize;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct CreationDateTime {
	creation_date: String,
}

impl Display for CreationDateTime {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		// TODO: stinks
		let parsed = chrono::DateTime::parse_from_rfc3339(&self.creation_date).unwrap();

		write!(
			f,
			"{}: {:02}.{:02}.{:04} {:02}:{:02}:{:02}",
			"Created".bold().bright_blue(),
			parsed.day(),
			parsed.month(),
			parsed.year(),
			parsed.hour(),
			parsed.minute(),
			parsed.second()
		)
	}
}
