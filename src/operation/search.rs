use crate::api::SEARCH_URL;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::ExtensionsList;
use crate::traits::runnable::Runnable;
use futures_util::StreamExt;
use reqwest::Client;

pub struct Search {
	search: ExtensionsList,
}

impl Search {
	async fn search_extension(slug: &str) -> Result<ExtensionsList> {
		Client::new()
			.get(SEARCH_URL)
			.query(&[("q", slug), ("page_size", "50"), ("app", "firefox"), ("lang", "en-US"), ("sort", "users")])
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))
	}
}

impl Search {
	pub async fn try_configure_from(slug: String) -> Result<Self> {
		Ok(Self { search: Self::search_extension(&slug).await? })
	}
}

impl Runnable for Search {
	async fn try_run(&self) -> Result<()> {
		futures_util::stream::iter(&self.search.extensions)
			.for_each(|extension| async move {
				println!("{}", extension);
			})
			.await;

		Ok(())
	}
}
