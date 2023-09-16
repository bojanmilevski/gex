use crate::errors::BrowserError;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Browser {
	name: String,
	pub path: PathBuf,
}

impl FromStr for Browser {
	type Err = BrowserError;

	fn from_str(browser_name: &str) -> Result<Self, Self::Err> {
		let home_str = std::env::var("HOME")?;
		let home = PathBuf::from(&home_str);

		let path = match browser_name {
			"firefox" => home.join(".mozilla/firefox"),
			"librewolf" => home.join(".librewolf"),
			_ => return Err(BrowserError::NotSupported),
		};

		match &path.exists() {
			true => Ok(Browser { name: browser_name.to_owned(), path }),
			false => Err(BrowserError::PathNotFound),
		}
	}
}
