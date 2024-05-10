use crate::addon::addon::Addon;
use crate::addon::response::Response;
use crate::api::ADDON_URL;
use crate::api::DOWNLOAD_URL;
use crate::cli::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::progress_bar::Bar;
use crate::traits::runnable::Runnable;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use colored::Colorize;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use reqwest::Client;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

pub struct Install {
	pub addons: Vec<Addon>,
	pub client: Client,
	pub configuration: Configuration,
}

impl Install {
	pub async fn find_addon(client: &Client, slug: &str) -> Result<Addon> {
		let response = client
			.get(format!("{ADDON_URL}/{slug}"))
			.send()
			.await?
			.json::<Response>()
			.await?;

		match response {
			Response::Addon(addon) => Ok(addon),
			_ => Err(anyhow!("{} not found.", slug)), // TODO: print a different message depending on variant?
		}
	}

	async fn install_addon(client: &Client, addon: &Addon) -> Result<Vec<u8>> {
		let version = addon.current_version.file.id;
		let name = addon.name.to_string();

		let response = client
			.get(format!("{DOWNLOAD_URL}/{version}"))
			.send()
			.await
			.with_context(|| format!("Error installing {name}."))?;

		let total_size = response
			.content_length()
			.context("Cannot get response content length.")?;

		let mut progress_bar = Bar::try_from(total_size)?;
		let mut bytes = Vec::new(); // TODO: avoid duplicating buffer
		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			progress_bar.update(chunk.len());
			bytes.extend_from_slice(&chunk); // TODO: avoid duplicating buffer
		}

		Ok(bytes)
	}
}

// FIX: configurable trait
impl Install {
	pub async fn try_configure_from(slugs: Vec<String>, cli_configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(cli_configuration)?;
		let client = Client::new();
		let addons = futures_util::stream::iter(slugs)
			.then(|slug| {
				let client = Arc::new(&client);
				async move { Self::find_addon(&client, &slug).await }
			})
			.try_collect()
			.await?;

		Ok(Self { client, configuration, addons })
	}
}

impl Runnable for Install {
	async fn try_run(&mut self) -> Result<()> {
		// TODO: prompt user if force reinstall
		let duplicates: Vec<&str> = self
			.addons
			.iter()
			.filter(|addon| self.configuration.database.contains(&addon.slug))
			.map(|addon| addon.slug.as_ref())
			.collect();

		let removed = self
			.configuration
			.database
			.remove_from_database(&duplicates)?;

		// TODO: super-addon
		let addon_map: Vec<(&Addon, Vec<u8>)> = futures_util::stream::iter(&self.addons)
			.then(|addon| {
				let client = Arc::new(&self.client);
				let name = addon.name.to_string();
				println!("{}: {name}", "Installing addon".bold().bright_blue());

				async move {
					match Self::install_addon(&client, addon).await {
						Ok(bytes) => Ok((addon, bytes)), // TODO: super addon is created here
						Err(err) => Err(err),
					}
				}
			})
			.try_collect()
			.await?;

		self.configuration
			.database
			.add(&addon_map, &self.configuration.profile)?;

		self.configuration
			.database
			.write(&self.configuration.profile)?;

		self.configuration
			.database
			.remove_from_disk(removed, &self.configuration.profile)?;

		// TODO: futures_util::stream::iter()
		for addon in &addon_map {
			let path = format!(
				"{}.xpi",
				self.configuration
					.profile
					.extensions
					.join(&addon.0.guid)
					.display()
			);

			tokio::fs::File::create(path)
				.await?
				.write_all(&addon.1)
				.await?;
		}

		Ok(())
	}
}
