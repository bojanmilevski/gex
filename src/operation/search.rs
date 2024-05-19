use super::api::SEARCH_URL;
use crate::addon::addon::Addons;
use crate::traits::runnable::Runnable;
use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;

pub struct Search {
	search: Addons,
}

impl Search {
	async fn search_addon(slug: &str) -> Result<Addons> {
		Ok(Client::new()
			.get(SEARCH_URL)
			.query(&[("q", slug), ("page_size", "50"), ("app", "firefox"), ("lang", "en-US"), ("sort", "users")])
			.send()
			.await?
			.json::<Addons>()
			.await?)
	}
}

// FIX: configurable trait
impl Search {
	pub async fn try_configure_from(slug: String) -> Result<Self> {
		let search = Self::search_addon(&slug).await?;

		Ok(Self { search })
	}
}

impl Runnable for Search {
	async fn try_run(&mut self) -> Result<()> {
		futures_util::stream::iter(&self.search.addons)
			.for_each(|addon| async move {
				println!("{}", addon);
			})
			.await;

		Ok(())
	}
}
