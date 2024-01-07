use super::configurable::Configurable;
use crate::api_url::API_URL;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::extension::extensions_list::ExtensionsList;
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Search {
	pub extensions: Vec<Extension>,
}

impl Search {
	async fn search_for_extension(slug: &str) -> Result<ExtensionsList> {
		Ok(reqwest::Client::new()
			.get(format!("{}/{}", API_URL, "search"))
			.query(&[("q", slug), ("page_size", "50"), ("app", "firefox"), ("lang", "en-US"), ("sort", "users")])
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))?)
	}
}

impl Configurable for Search {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		if cli.operation.search.is_none() {
			return Ok(Self { ..Default::default() });
		}

		let slug = cli.operation.search.as_ref().unwrap().as_str();
		let extensions = Self::search_for_extension(slug).await?.extensions;

		Ok(Self { extensions })
	}
}
