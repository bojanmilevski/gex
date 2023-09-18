use crate::errors::ProfileError;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Profile {
	pub name: String,
	pub path: PathBuf,
}

impl Profile {
	pub async fn from(profile_name: &str, browser_data_path: &PathBuf) -> Result<Profile, ProfileError> {
		let ini_file = browser_data_path.join("profiles.ini");
		let config = ini::Ini::load_from_file(&ini_file)?;

		// this is ugly... i need to fix asap
		let path_slug = config
			.iter()
			.find_map(|(sec, prop)| {
				sec.and_then(|s| {
					if s.starts_with("Profile") {
						prop.get("Name")
							.filter(|&v| v == profile_name)
							.and_then(|_| prop.get("Path"))
					} else {
						None
					}
				})
			})
			.ok_or(ProfileError::ProfileNotFound)?;

		let path = browser_data_path.join(&path_slug).join("extensions");

		if !path.exists() {
			std::fs::create_dir(&path)?;
		}

		Ok(Profile { name: profile_name.to_owned(), path })
	}
}
