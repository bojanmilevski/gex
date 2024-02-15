use super::browser::Browser;
use super::database::Database;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use crate::operation::configurable::Configurable;
use ini::Ini;
use std::path::PathBuf;

pub struct Profile {
	pub browser: Browser,
	pub database: Database,
	pub name: String,
	pub path: PathBuf,
}

impl Profile {
	fn get_profile_in_use(ini: &Ini) -> Result<&str> {
		ini.sections()
			.flatten()
			.filter(|section| section.starts_with("Install"))
			.find_map(|section| ini.get_from(Some(section), "Default"))
			.ok_or(Error::ProfileNotFound("TODO".to_owned()))
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

impl Configurable for Profile {
	async fn try_configure_from(cli: Cli) -> Result<Self> {
		let browser = Browser::try_from(cli.clone())?;
		let profiles = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(profiles)?;

		let name = match cli.profile {
			Some(profile) => Self::get_specified_profile(&ini, profile)?,
			None => Self::get_profile_in_use(&ini)?,
		}
		.to_owned();

		let path = browser.path.join(&name);

		if !path.exists() {
			// std::fs::create_dir(&path)?;
			return Err(Error::ProfileNotFound(name));
		}

		let database = Database::try_configure_from(&path).await?;

		Ok(Self { browser, path, name, database })
	}
}
