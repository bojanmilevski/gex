use crate::args::Args;
use crate::errors::BrowserError;
use crate::Configurable;

use std::path::PathBuf;

use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Browser {
	pub name: String,
	pub path: PathBuf,
}

#[async_trait]
impl Configurable for Browser {
	type Err = BrowserError;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err> {
		let home_str = std::env::var("HOME")?;
		let home = PathBuf::from(&home_str);

		let path = match args.browser.as_str() {
			"firefox" => home.join(".mozilla/firefox"),
			"librewolf" => home.join(".librewolf"),
			"firedragon" => home.join(".firedragon"),
			_ => return Err(BrowserError::NotSupported),
		};

		if !path.exists() {
			return Err(BrowserError::PathNotFound);
		}

		let browser = Browser { name: args.browser.to_string(), path };

		Ok(browser)
	}
}
