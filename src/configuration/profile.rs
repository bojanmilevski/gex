use crate::cli::CliConfiguration;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use ini::Ini;
use std::path::PathBuf;

pub struct Profile {
	pub browser_path: PathBuf,
	pub name: String,
	pub path: PathBuf,
	pub extensions: PathBuf,
	pub extensions_json: PathBuf,
	pub addons_json: PathBuf,
}

impl Profile {
	fn get_browser_path(browser: &str) -> Result<PathBuf> {
		let home = home::home_dir().context("Cannot find user HOME path")?;

		let path = match browser {
			"firefox" => ".mozilla/firefox",
			"librewolf" => ".librewolf",
			"firedragon" => ".firedragon",
			_ => {
				return Err(anyhow!(
					"Browser {} is currently not supported. Please open an issue here: https://github.com/bojanmilevski/gex/issues",
					browser
				))
			}
		};

		let path = home.join(path);

		if !path.exists() {
			return Err(anyhow!("{} path ({}) does not exist.", browser, path.display()));
		}

		Ok(path)
	}

	fn get_profile_in_use(ini: &Ini) -> Result<&str> {
		ini.sections()
			.flatten()
			.filter(|section| section.starts_with("Install"))
			.find_map(|section| ini.get_from(Some(section), "Default"))
			.context("Profile in use in profiles.ini does not exist.")
	}

	fn get_specified_profile<'a>(ini: &'a Ini, profile: &str) -> Result<&'a str> {
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
			.with_context(|| format!("Profile {profile} not found"))
	}
}

impl TryFrom<CliConfiguration> for Profile {
	type Error = anyhow::Error;

	fn try_from(configuration: CliConfiguration) -> Result<Self> {
		let browser_path = Self::get_browser_path(&configuration.browser)?;
		let profiles = browser_path.join("profiles.ini");
		let ini = Ini::load_from_file(profiles)?;

		let name = match configuration.profile {
			Some(profile) => Self::get_specified_profile(&ini, &profile)?,
			None => Self::get_profile_in_use(&ini)?,
		}
		.to_string();

		let path = browser_path.join(&name);
		let extensions = path.join("extensions");
		let extensions_json = path.join("extensions.json");
		let addons_json = path.join("addons.json");

		Ok(Self { browser_path, name, path, extensions, extensions_json, addons_json })
	}
}
