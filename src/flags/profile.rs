use crate::args::Args;
use crate::errors::Error;
use crate::errors::Result;
use crate::flags::Browser;
use crate::Configurable;
use async_trait::async_trait;
use ini::Ini;
use rayon::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Profile {
	browser: Browser,
	pub name: String,
	pub path: PathBuf,
}

#[async_trait]
impl Configurable for Profile {
	type Err = Error;

	async fn configure_from(args: &Args) -> Result<Self> {
		if !args.search.is_empty() {
			return Ok(Self { ..Default::default() });
		}

		let browser = Browser::configure_from(&args).await?;
		let ini_file = browser.path.join("profiles.ini");
		let config = Ini::load_from_file(&ini_file)?;

		/*
			this piece of code is a disaster

			also, find a way to replace .iter() with .par_iter()
			rayon and tokio async runtimes come into clash here
		*/
		let path_slug = config
			.iter()
			.find_map(|(sector, property)| {
				sector.and_then(|sec| {
					if sec.starts_with("Profile") {
						property
							.get("Name")
							.filter(|&val| val == &args.profile)
							.and_then(|_| property.get("Path"))
					} else {
						None
					}
				})
			})
			.ok_or(Error::ProfileNotFound)?;

		let path = browser.path.join(&path_slug).join("extensions");

		if !path.exists() {
			tokio::fs::create_dir(&path).await?;
		}

		let name = args.profile.to_owned();

		Ok(Self { browser, name, path })
	}
}
