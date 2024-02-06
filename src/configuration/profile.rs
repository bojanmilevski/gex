use super::browser::Browser;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use ini::Ini;
use std::path::PathBuf;

pub struct Profile {
	pub browser: Browser,
	pub path: PathBuf,
	pub name: String,
}

impl Profile {
	fn get_profile_in_use(ini: &Ini) -> Result<&str> {
		Ok(ini
			.sections()
			.flatten()
			.filter(|section| section.starts_with("Install"))
			.find_map(|section| ini.get_from(Some(section), "Default"))
			.ok_or(Error::ProfileNotFound("TODO".to_owned()))?)
	}

	fn get_specified_profile(ini: &Ini, profile: String) -> Result<&str> {
		Ok(ini
			.sections()
			.flatten()
			.filter(|section| section.starts_with("Profile"))
			.find_map(|section| {
				if ini.get_from(Some(section), "Name")? == profile {
					ini.get_from(Some(section), "Path")
				} else {
					None
				}
			})
			.ok_or(Error::ProfileNotFound(profile))?)
	}
}

impl TryFrom<&Cli> for Profile {
	type Error = Error;

	fn try_from(cli: &Cli) -> Result<Self> {
		let browser = Browser::try_from(cli)?;
		let profiles_file = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(&profiles_file)?;

		let path_slug = match &cli.profile {
			Some(profile) => Self::get_specified_profile(&ini, profile.to_owned())?,
			None => Self::get_profile_in_use(&ini)?,
		};

		let path = browser.path.join(&path_slug);

		if !path.exists() {
			std::fs::create_dir(&path)?;
		}

		Ok(Self { browser, path, name: path_slug.to_owned() })
	}
}
