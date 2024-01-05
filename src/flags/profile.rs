use crate::args::Args;
use crate::errors::Error;
use crate::errors::Result;
use crate::flags::browser::Browser;
use crate::flags::configurable::Configurable;
use ini::Ini;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
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
			.ok_or(Error::ProfileNotFound)?
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
			.ok_or(Error::ProfileNotFound)?
			.to_owned())
	}
}

impl Configurable for Profile {
	async fn configure_from(args: &Args) -> Result<Self> {
		if args.search.is_some() {
			return Ok(Self { ..Default::default() });
		}

		let browser = Browser::configure_from(&args).await?;

		let profiles_file = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(&profiles_file)?;

		let path_slug = match &args.profile {
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

impl Display for Profile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "TODO: impl Display for Profile")
	}
}

impl Into<String> for Profile {
	fn into(self) -> String {
		String::from("TODO: impl Into<String> for Profile")
	}
}
