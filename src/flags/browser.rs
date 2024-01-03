use crate::args::Args;
use crate::errors::Error;
use crate::errors::Result;
use crate::Configurable;
use home::home_dir;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Browser {
	pub name: String,
	pub path: PathBuf,
}

impl Configurable for Browser {
	async fn configure_from(args: &Args) -> Result<Self> {
		if args.search.is_some() {
			return Ok(Self { ..Default::default() });
		}

		let home_str = home_dir().ok_or(Error::HomeVar)?;
		let home = PathBuf::from(&home_str);

		let path = match args.browser.as_str() {
			"firefox" => home.join(".mozilla/firefox"),
			"librewolf" => home.join(".librewolf"),
			"firedragon" => home.join(".firedragon"),
			_ => return Err(Error::BrowserNotSupported),
		};

		if !path.exists() {
			return Err(Error::BrowserPathNotFound);
		}

		Ok(Self { name: args.browser.to_string(), path })
	}
}
