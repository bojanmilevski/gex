use crate::args::Args;
use crate::errors::Error;
use crate::errors::Result;
use crate::flags::Browser;
use crate::Configurable;
use async_trait::async_trait;
use ini::Ini;
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

	async fn get_specified_profile(ini: &Ini, profile_name: &str) -> Result<String> {
		Ok(ini
			.sections()
			.flatten()
			.filter(|section| section.starts_with("Profile"))
			.find_map(|section| {
				if ini.get_from(Some(section), "Name")? == profile_name {
					ini.get_from(Some(section), "Path")
				} else {
					None
				}
			})
			.ok_or(Error::ProfileNotFound)?
			.to_owned())
	}
}

#[async_trait]
impl Configurable for Profile {
	type Err = Error;

	async fn configure_from(args: &Args) -> Result<Self> {
		if args.search.is_some() {
			return Ok(Self { ..Default::default() });
		}

		let browser = Browser::configure_from(&args).await?;

		let profiles_file = browser.path.join("profiles.ini");
		let ini = Ini::load_from_file(&profiles_file)?;

		let path_slug = match &args.profile {
			Some(profile_name) => Self::get_specified_profile(&ini, &profile_name).await?,
			None => Self::get_profile_in_use(&ini).await?,
		};

		let path = browser.path.join(&path_slug);

		if !path.exists() {
			tokio::fs::create_dir(&path).await?;
		}

		Ok(Self { browser, path })
	}
}
