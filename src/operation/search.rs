use super::runnable::Runnable;
use crate::api::SEARCH_URL;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extensions_list::ExtensionsList;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
	search: ExtensionsList,
}

impl Search {
	async fn search_extension(slug: &str) -> Result<ExtensionsList> {
		Ok(reqwest::Client::new()
			.get(SEARCH_URL)
			.query(&[("q", slug), ("page_size", "50"), ("app", "firefox"), ("lang", "en-US"), ("sort", "users")])
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))?)
	}
}

impl Search {
	pub async fn try_configure_from(val: String, profile: Profile) -> Result<Self> {
		Ok(Self { search: Self::search_extension(&val).await? })
	}
}

impl Runnable for Search {
	async fn try_run(&self) -> Result<()> {
		for extension in &self.search.extensions {
			println!("{}", extension);
		}

		Ok(())
	}
}
