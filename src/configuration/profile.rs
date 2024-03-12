use super::browser::Browser;
use crate::cli::Configuration;
use crate::errors::Error;
use crate::errors::Result;
use ini::Ini;
use std::path::PathBuf;

pub struct Profile {
	pub name: String,
	pub path: PathBuf,
	pub browser: Browser,
}

impl Profile {
	fn get_profile_in_use(ini: &Ini) -> Result<&str> {
		ini.sections()
			.flatten()
			.filter(|section| section.starts_with("Install"))
			.find_map(|section| ini.get_from(Some(section), "Default"))
			.ok_or(Error::ProfileNotFound(
				"Path for profile in use does not exist.".to_owned(),
			))
	}

	fn get_specified_profile(ini: &Ini, profile: String) -> Result<&str> {
		ini.sections()
			.flatten()
			.filter(|section| section.starts_with("Profile"))
			.find_map(|section| {
				if ini.get_from(Some(section), "Name")? == profile {
					ini.get_from(Some(section), "Path")
				} else {
					None
				}
			})
			.ok_or(Error::ProfileNotFound(profile))
	}
}

impl TryFrom<Configuration> for Profile {
	type Error = Error;

	fn try_from(configuration: Configuration) -> Result<Self> {
		let browser = Browser::try_from(&configuration)?;
		let profiles = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(profiles)?;

		let name = match configuration.profile {
			Some(profile) => Self::get_specified_profile(&ini, profile)?,
			None => Self::get_profile_in_use(&ini)?,
		}
		.to_owned();

		let path = browser.path.join(&name);

		if !path.exists() {
			return Err(Error::ProfileNotFound(name));
		}

		Ok(Self {
			browser,
			path,
			name,
		})
	}
}
