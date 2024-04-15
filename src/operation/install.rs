use crate::addon::addon::Addon;
use crate::api::ADDON_URL;
use crate::api::DOWNLOAD_URL;
use crate::cli::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::errors::Error;
use crate::errors::Result;
use crate::progress_bar::Bar;
use crate::traits::runnable::Runnable;
use colored::Colorize;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

pub struct Install {
	pub addons: Vec<Addon>,
	pub client: Client,
	pub configuration: Configuration,
}

impl Install {
	pub async fn find_addon(client: &Client, slug: &str) -> Result<Addon> {
		Ok(client
			.get(format!("{ADDON_URL}/{slug}"))
			.send()
			.await?
			.json::<Addon>()
			.await?)
	}

	async fn install_addon(client: &Client, addon: &Addon) -> Result<Vec<u8>> {
		let version = addon.current_version.file.id;
		let name = addon.name.to_string();

		let response = client
			.get(format!("{}/{}", DOWNLOAD_URL, version))
			.send()
			.await
			.or(Err(Error::Install(String::from(&name))))?;

		let total_size = response.content_length().unwrap();
		let mut progress_bar = Bar::from(total_size);
		let mut bytes = Vec::new(); // FIX:
		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			progress_bar.update(chunk.len());
			bytes.extend_from_slice(&chunk); // FIX:
		}

		Ok(bytes)
	}
}

impl Install {
	pub async fn try_configure_from(slugs: Vec<String>, cli_configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(cli_configuration)?;
		let client = Client::new();
		let addons = futures_util::stream::iter(slugs)
			.then(|slug| {
				let client = client.clone();
				async move { Self::find_addon(&client, &slug).await }
			})
			.try_collect()
			.await?;

		Ok(Self { client, configuration, addons })
	}
}

impl Runnable for Install {
	async fn try_run(&mut self) -> Result<()> {
		// FIX: prompt user if force reinstall
		let duplicates: Vec<&str> = self
			.addons
			.iter()
			.filter(|addon| self.configuration.database.contains(&addon.slug))
			.map(|addon| addon.guid.as_ref())
			.collect();

		self.configuration
			.database
			.remove_from_database(&duplicates)?;

		let addon_map: Vec<(&Addon, Vec<u8>)> = futures_util::stream::iter(&self.addons)
			.then(|addon| {
				let client = self.client.clone();
				let name = addon.name.to_string();
				println!("{}: {}", "Installing addon".bold().bright_blue(), name);

				async move {
					match Self::install_addon(&client, addon).await {
						Ok(bytes) => Ok((addon, bytes)),
						Err(err) => {
							eprintln!("{}: {}", "Error installing addon".bold().red(), err);
							Err(err)
						}
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
			.remove_from_disk(&duplicates, &self.configuration.profile)?;

		let extensions_path = self.configuration.profile.path.join("extensions");

		/* if !extensions_path.exists() {
			tokio::fs::create_dir(&extensions_path).await?;
		} */

		// FIX: futures_util::stream::iter()
		for addon in &addon_map {
			let path = format!("{}.xpi", extensions_path.join(&addon.0.guid).display());
			let mut file = tokio::fs::File::create(path).await?;
			file.write_all(&addon.1).await?;
		}

		Ok(())
	}
}
