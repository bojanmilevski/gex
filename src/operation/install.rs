use super::api::ADDON_URL;
use crate::addon::Addon;
use crate::addon::Response;
use crate::cli::CliConfiguration;
use crate::database::database::Database;
use crate::database::manifests::manifest::Manifest;
use crate::progress_bar::Bar;
use crate::traits::Runnable;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use colored::Colorize;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use reqwest::Client;
use std::sync::Arc;

pub struct Install {
	pub addons: Vec<Addon>,
	pub client: Client,
	pub database: Database,
}

// FIX: rename struct
pub struct Package<'a> {
	pub json_response: &'a Addon,
	pub xpi: Vec<u8>,
	pub manifest: Manifest,
}

impl Install {
	pub async fn find_addon(client: &Client, slug: &str) -> Result<Addon> {
		let response = client
			.get(format!("{}/{}", ADDON_URL, slug))
			.send()
			.await?
			.json::<Response>()
			.await?;

		match response {
			Response::Addon(addon) => Ok(addon),
			_ => Err(anyhow!("{} not found.", slug)), // FIX: print a different message depending on variant?
		}
	}

	async fn install_addon(client: &Client, addon: &Addon) -> Result<Vec<u8>> {
		let response = client
			.get(addon.current_version.file.url.as_ref())
			.send()
			.await
			.with_context(|| format!("Error installing {}.", addon.name))?;

		let total_size = response
			.content_length()
			.context("Cannot get response content length.")?;

		let mut progress_bar = Bar::try_from(total_size)?;
		let mut bytes = Vec::new(); // FIX: avoid duplicating buffer
		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			progress_bar.update(chunk.len());
			bytes.extend_from_slice(&chunk); // FIX: avoid duplicating buffer
		}

		Ok(bytes)
	}
}

// FIX: init trait
impl Install {
	pub async fn try_init(slugs: Vec<String>, cli_configuration: CliConfiguration) -> Result<Self> {
		let database = Database::try_from(cli_configuration)?;
		let client = Client::new();
		let addons = futures_util::stream::iter(slugs)
			.then(|slug| {
				let client = Arc::new(&client);
				async move { Self::find_addon(&client, &slug).await }
			})
			.try_collect()
			.await?;

		Ok(Self { client, database, addons })
	}
}

impl Runnable for Install {
	async fn try_run(&mut self) -> Result<()> {
		// // FIX: if some addons are already installed, prompt user if they wish to force reinstall
		// let duplicates: Vec<&str> = self
		// 	.addons
		// 	.iter()
		// 	.filter(|addon| self.database.contains(&addon.slug)) // FIX: there should be a more efficient way of doing this
		// 	.map(|addon| addon.slug.as_ref())
		// 	.collect();

		// let removed = self.database.remove_from_database(&duplicates)?;

		// FIX: shorten this huge abomination. make a separate fn
		let addons: Vec<Package> = futures_util::stream::iter(&self.addons)
			.then(|addon| {
				let client = Arc::new(&self.client);
				println!("{}: {}", "Installing addon".bold().bright_blue(), addon.name);

				async move {
					match Self::install_addon(&client, addon).await {
						Ok(bytes) => {
							// FIX: make try_from fn for this:
							let package = Package { json_response: addon, manifest: Manifest::try_from(&bytes)?, xpi: bytes };
							Ok(package)
						}

						Err(err) => Err(err),
					}
				}
			})
			.try_collect()
			.await?;

		self.database.add(&addons)?;
		self.database.write_to_disk()?;
		self.database.remove_from_disk(&addons)?;
		self.database.write_new_addons_to_disk(&addons).await?;

		Ok(())
	}
}
