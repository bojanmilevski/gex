use crate::addon::addon::Addons;
use crate::api::SEARCH_URL;
use crate::errors::Error;
use crate::errors::Result;
use crate::traits::runnable::Runnable;
use futures_util::StreamExt;
use reqwest::Client;

pub struct Search {
	search: Addons,
}

impl Search {
	async fn search_addon(slug: &str) -> Result<Addons> {
		Client::new()
			.get(SEARCH_URL)
			.query(&[("q", slug), ("page_size", "50"), ("app", "firefox"), ("lang", "en-US"), ("sort", "users")])
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json()
			.await
			.or(Err(Error::AddonNotFound(slug.to_owned())))
	}
}

impl Search {
	pub async fn try_configure_from(slug: String) -> Result<Self> {
		let search = Self::search_addon(&slug).await?;

		Ok(Self { search })
	}
}

impl Runnable for Search {
	async fn try_run(&self) -> Result<()> {
		futures_util::stream::iter(&self.search.addons)
			.for_each(|addon| async move {
				println!("{}", addon);
			})
			.await;

		Ok(())
	}
}
