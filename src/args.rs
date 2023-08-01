use crate::errors;
use crate::profile;

use errors::ArgsError;
use errors::ProfileError;
use profile::Profile;

use clap::Parser;
use ini::Ini;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, num_args = 1.., value_delimiter = ' ')]
	pub install: Vec<String>,

	#[arg(short, long, default_value = "default-release")]
	pub profile: String,

	#[arg(short, long, default_value = "firefox")]
	pub browser: String,
}

impl Args {
	pub async fn get_profiles(&self) -> Result<Vec<Profile>, ProfileError> {
		let browser_data_path = self.get_browser_data_path().await?;
		let ini = format!("{}/profiles.ini", &browser_data_path);
		let config = Ini::load_from_file(ini)?;
		let profiles: Vec<Profile> = config
			.iter()
			.filter_map(|(sector, property)| {
				sector.and_then(|sec| {
					if sec.starts_with("Profile") {
						let name = property.get("Name")?.to_string();
						let path = property.get("Path")?.to_string();
						Some(Profile::from(name, path))
					} else {
						None
					}
				})
			})
			.collect();

		Ok(profiles)
	}

	pub async fn validate_args(self) -> Result<Self, ArgsError> {
		let profiles = self.get_profiles().await?;
		let names: Vec<String> = profiles.iter().map(|profile| profile.name.clone()).collect();
		if !names.contains(&self.profile) {
			eprintln!("Profile {} does not exist.", &self.profile);
			return Err(ArgsError::ProfileNotFound);
		}

		Ok(self)
	}

	pub async fn get_browser_data_path(&self) -> Result<String, std::env::VarError> {
		let home = std::env::var("HOME")?;
		let mut path = String::new();

		if &self.browser == "firefox" {
			path = format!("{}/.mozilla/{}", home, &self.browser);
		} else if &self.browser == "librewolf" {
			path = format!("{}/.{}", home, &self.browser);
		}

		Ok(path)
	}

	pub async fn get_download_path(&self) -> Result<String, ProfileError> {
		let browser_data_path = &self.get_browser_data_path().await?;
		let profiles = self.get_profiles().await?;
		let profile: &Profile = profiles.iter().find(|p| p.name == self.profile).unwrap();
		Ok(format!("{}/{}/extensions", &browser_data_path, &profile.path))
	}
}
