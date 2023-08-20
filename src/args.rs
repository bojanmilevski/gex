use std::path::Path;

use crate::errors;
use crate::profile;

use errors::ProfileError;
use profile::Profile;

use clap::Parser;
use ini::Ini;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true)]
	pub install: Vec<String>,

	#[arg(short, long, default_value = "default-release", required = false)]
	pub profile: String,

	#[arg(short, long, default_value = "firefox", value_parser = ["firefox", "librewolf"], required = false)]
	pub browser: String,
}

impl Args {
	pub async fn get_profile(&self) -> Result<Profile, ProfileError> {
		let browser_data_path = self.get_browser_data_path().await?;
		let ini = format!("{}/profiles.ini", &browser_data_path);
		let config = Ini::load_from_file(ini)?;

		config
			.iter()
			.filter_map(|(sector, property)| {
				sector
					.filter(|sec| sec.starts_with("Profile"))
					.and_then(|_| {
						let name = property.get("Name")?.to_string();
						let path = property.get("Path")?.to_string();
						Some(Profile::new(name, path))
					})
			})
			.find(|profile| &profile.name == &self.profile)
			.ok_or(ProfileError::ProfileNotFound)
	}

	pub async fn get_browser_data_path(&self) -> Result<String, ProfileError> {
		let home = std::env::var("HOME")?;

		let path = match self.browser.as_str() {
			"firefox" => format!("{}/.mozilla/{}", home, &self.browser),
			"librewolf" => format!("{}/.{}", home, &self.browser),
			_ => return Err(ProfileError::BrowserNotSupported),
		};

		match Path::exists(Path::new(&path)) {
			true => Ok(path),
			false => Err(ProfileError::BrowserPathNotFound),
		}
	}

	pub async fn get_download_path(&self) -> Result<String, ProfileError> {
		let browser_data_path = &self.get_browser_data_path().await?;
		let profile = self.get_profile().await?;
		Ok(format!("{}/{}/extensions", &browser_data_path, &profile.path))
	}
}
