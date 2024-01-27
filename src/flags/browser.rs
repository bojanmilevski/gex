use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use home::home_dir;
use std::path::PathBuf;

pub struct Browser {
	pub name: String,
	pub path: PathBuf,
}

impl Configurable for Browser {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		let home_str = home_dir().ok_or(Error::Home)?;
		let home = PathBuf::from(&home_str);
		let browser = cli.browser.to_owned();

		let path = match cli.browser.as_str() {
			"firefox" => home.join(".mozilla/firefox"),
			"librewolf" => home.join(".librewolf"),
			"firedragon" => home.join(".firedragon"),
			_ => return Err(Error::BrowserNotSupported(browser)),
		};

		if !path.exists() {
			return Err(Error::BrowserPathNotFound(browser));
		}

		Ok(Self { name: cli.browser.to_string(), path })
	}
}
