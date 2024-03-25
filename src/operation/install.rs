use crate::addon::addon::Addon;
use crate::api::ADDON_URL;
use crate::api::DOWNLOAD_URL;
use crate::cli::Configuration as CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::errors::Error;
use crate::errors::Result;
use crate::progress_bar::Bar;
use crate::traits::runnable::Runnable;
use colored::Colorize;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use url::Url;

pub struct Install {
	client: Client,
	configuration: Configuration,
	pub addons: Vec<Addon>,
}

impl Install {
	pub async fn find_addon(client: &Client, slug: &str) -> Result<Addon> {
		client
			.get(Url::parse(ADDON_URL)?.join(slug)?)
			.send()
			.await
			.or(Err(Error::Query(String::from(slug))))?
			.json::<Addon>()
			.await
			.or(Err(Error::AddonNotFound(String::from(slug))))
	}

	async fn install_addon(client: &Client, addon: &Addon) -> Result<Vec<u8>> {
		let version = addon.current_version.file.id;
		let name = addon.get_name();

		let response = client
			.get(Url::parse(DOWNLOAD_URL)?.join(&version.to_string())?)
			.send()
			.await
			.or(Err(Error::Install(String::from(&name))))?;

		let total_size = response.content_length().unwrap();
		let mut progress_bar = Bar::from(total_size);
		let mut bytes = Vec::new();
		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			progress_bar.update(chunk.len());
			bytes.extend_from_slice(&chunk);
		}

		Ok(bytes)
	}
}

impl Install {
	pub async fn try_configure_from(val: Vec<String>, configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(configuration)?;
		let client = Client::new();

		let mut addons = Vec::new();

		for addon in val {
			match Self::find_addon(&client, &addon).await {
				Ok(ext) => addons.push(ext),
				Err(err) => return Err(err),
			};
		}

		// TODO: I WILL HAVE MY REVENGE
		// let addons = futures_util::stream::iter(val.into_iter())
		// .map(|addon| async move { Self::find_addon(&client, &addon).await })
		// .try_collect::<Vec<Addon>>()
		// .await?;

		Ok(Self { client, configuration, addons })
	}
}

impl Runnable for Install {
	async fn try_run(&mut self) -> Result<()> {
		// TODO: futures_util::stream::iter

		let mut addon_map: Vec<(&Addon, Vec<u8>)> = Vec::new();
		for addon in &self.addons {
			let name = addon.get_name();
			match Self::install_addon(&self.client, addon).await {
				Ok(bytes) => {
					println!("{}: {}", "Installing addon".bold().bright_blue(), name);
					println!("{}", addon);
					addon_map.push((addon, bytes));
				}

				Err(err) => {
					eprintln!("{}: {}", "Error installing addon".bold().red(), err);
					return Err(err);
				}
			}
		}

		for addon in &addon_map {
			if let Err(err) = &self
				.configuration
				.database
				.add(addon.0, &addon.1, &self.configuration.profile)
			{
				eprintln!("{}: {}", "Error adding addon to database".bold().red(), err);
			}
		}

		let extensions_path = self.configuration.profile.path.join("extensions");
		if !extensions_path.exists() {
			tokio::fs::create_dir(&extensions_path).await?;
		}

		if let Err(err) = &self
			.configuration
			.database
			.write(&self.configuration.profile)
		{
			eprintln!("{}: {}", "Error writing to database".bold().red(), err);
		}

		for addon in addon_map {
			let path = format!("{}.xpi", extensions_path.join(&addon.0.guid).display());
			let mut file = tokio::fs::File::create(path).await?;
			file.write_all(&addon.1).await?;
		}

		Ok(())
	}
}
