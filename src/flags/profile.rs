use super::browser::Browser;
use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use ini::Ini;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Profile {
	pub browser: Browser,
	// pub name: String,
	pub path: PathBuf,
}

impl Profile {
	async fn get_profile_in_use(ini: &Ini) -> Result<String> {
		Ok(ini
			.sections()
			.flatten()
			.filter(|section| section.starts_with("Install"))
			.find_map(|section| ini.get_from(Some(section), "Default"))
			.ok_or(Error::ProfileNotFound("in use".to_owned()))?
			.to_owned())
	}

	async fn get_specified_profile(ini: &Ini, profile: &str) -> Result<String> {
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
			.ok_or(Error::ProfileNotFound(profile.to_owned()))?
			.to_owned())
	}
}

impl Configurable for Profile {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		let browser = Browser::try_configure_from(&cli).await?;
		let profiles_file = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(&profiles_file)?;

		let path_slug = match &cli.profile {
			Some(profile) => Self::get_specified_profile(&ini, &profile).await?,
			None => Self::get_profile_in_use(&ini).await?,
		};

		let path = browser.path.join(&path_slug);

		if !path.exists() {
			tokio::fs::create_dir(&path).await?;
		}

		Ok(Self { browser, path })
	}
}
