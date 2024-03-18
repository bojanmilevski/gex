use crate::addon::addon::Addon;
use crate::api::ADDON_URL;
use crate::api::DOWNLOAD_URL;
use crate::configuration::configuration::Configuration;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::manifest::manifest::Manifest;
use crate::progress_bar::Bar;
use crate::traits::runnable::Runnable;
use colored::Colorize;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

pub struct Install {
	client: Client,
	configuration: Configuration,
	pub addons: Vec<Addon>,
}

impl Install {
	pub async fn find_addon(client: &Client, slug: &str) -> Result<Addon> {
		client
			.get(format!("{ADDON_URL}/{slug}"))
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json::<Addon>()
			.await
			.or(Err(Error::AddonNotFound(slug.to_owned())))
	}

	async fn install_addon(client: &Client, addon: &Addon, profile: &Profile) -> Result<()> {
		let version = addon.current_version.file.id;
		let guid = addon.guid.clone();
		let name = addon.get_name();

		let response = client
			.get(format!("{DOWNLOAD_URL}/{version}"))
			.send()
			.await
			.or(Err(Error::Install(name.clone())))?;

		let addons_folder = profile.path.join("extensions");

		if !addons_folder.exists() {
			tokio::fs::create_dir(&addons_folder).await?;
		}

		let path = format!("{}.xpi", addons_folder.join(guid).display());
		let mut file = tokio::fs::File::create(path).await?;

		let total_size = response
			.content_length()
			.ok_or(Error::ContentLength(name))?;

		let mut bar = Bar::from(total_size);
		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			file.write_all(&chunk).await?;
			bar.update(chunk.len());
		}

		Ok(())
	}
}

impl Install {
	pub async fn try_configure_from(val: Vec<String>, configuration: crate::cli::Configuration) -> Result<Self> {
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
	async fn try_run(&self) -> Result<()> {
		futures_util::stream::iter(&self.addons)
			.for_each(|addon| async move {
				let name = addon.get_name();
				println!("{}: {}", "Installing addon".bold().bright_blue(), name);
				println!("{}", addon);

				match Self::install_addon(&self.client, addon, &self.configuration.profile).await {
					Ok(_) => {
						if let Err(err) = Manifest::add_addon_to_database(&self.configuration.profile, addon) {
							eprintln!("{}: Manifest error: {}", "Error".bold().red(), err);
						};
					}

					Err(err) => {
						eprintln!("{}: {}", "Error".bold().red(), err);
						eprintln!("{}: {}", "Error installing addon".bold().red(), name);
					}
				};
			})
			.await;

		Ok(())
	}
}
