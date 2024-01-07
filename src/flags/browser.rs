use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use home::home_dir;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Browser {
	pub name: String,
	pub path: PathBuf,
}

impl Configurable for Browser {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		let home_str = home_dir().ok_or(Error::Home)?;
		let home = PathBuf::from(&home_str);

		let path = match cli.browser.as_str() {
			"firefox" => home.join(".mozilla/firefox"),
			"librewolf" => home.join(".librewolf"),
			"firedragon" => home.join(".firedragon"),
			_ => return Err(Error::BrowserNotSupported(cli.browser.clone())),
		};

		if !path.exists() {
			return Err(Error::BrowserPathNotFound(cli.browser.clone()));
		}

		Ok(Self { name: cli.browser.to_string(), path })
	}
}
