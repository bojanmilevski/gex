use crate::args::Args;
use crate::errors::ProfileError;
use crate::flags::Browser;
use crate::Configurable;

use std::path::PathBuf;

use async_trait::async_trait;

#[derive(Debug, Default, Clone)]
pub struct Profile {
	pub browser: Browser,
	pub name: String,
	pub path: PathBuf,
}

#[async_trait]
impl Configurable for Profile {
	type Err = ProfileError;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err> {
		if !args.search.is_empty() {
			return Ok(Self { ..Default::default() });
		}

		let browser = Browser::configure_from(&args).await?;

		/*
			browser configure_from will be run 2 times during flags construction
			this is a very very very stupid workaround
		*/
		let ini_file = browser.path.join("profiles.ini");
		let config = ini::Ini::load_from_file(&ini_file)?;

		/*
			this piece of code is a disaster

			also, find a way to replace .iter() with .par_iter()
			rayon and tokio async runtimes come into clash here
		*/
		let path_slug = config
			.iter()
			.find_map(|(sec, prop)| {
				sec.and_then(|s| {
					if s.starts_with("Profile") {
						prop.get("Name")
							.filter(|&v| v == &args.profile)
							.and_then(|_| prop.get("Path"))
					} else {
						None
					}
				})
			})
			.ok_or(ProfileError::ProfileNotFound)?;

		let path = browser.path.join(&path_slug).join("extensions");

		if !path.exists() {
			std::fs::create_dir(&path)?;
		}

		let name = args.profile.to_owned();

		Ok(Self { browser, name, path })
	}
}
