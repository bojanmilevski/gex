use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use std::path::PathBuf;

pub struct Browser {
	pub name: String,
	pub path: PathBuf,
}

impl TryFrom<Cli> for Browser {
	type Error = Error;

	fn try_from(cli: Cli) -> Result<Self> {
		let home = home::home_dir().ok_or(Error::Home)?;
		let name = cli.browser.to_owned();

		let browser_path = match name.as_str() {
			"firefox" => ".mozilla/firefox",
			"librewolf" => ".librewolf",
			"firedragon" => ".firedragon",
			_ => return Err(Error::BrowserNotSupported(name)),
		};

		let path = home.join(browser_path);

		if !path.exists() {
			return Err(Error::BrowserPathNotFound(name));
		}

		Ok(Self { name, path })
	}
}
