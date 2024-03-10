use crate::api::DOWNLOAD_URL;
use crate::api::EXTENSION_URL;
use crate::configuration::configuration::Configuration;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::manifest::manifest::Manifest;
use crate::progress_bar::Bar;
use crate::runnable::Runnable;
use colored::Colorize;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

pub struct Install {
	client: Client,
	configuration: Configuration,
	pub extensions: Vec<Extension>,
}

impl Install {
	pub async fn find_extension(client: &Client, slug: &str) -> Result<Extension> {
		client
			.get(format!("{EXTENSION_URL}/{slug}"))
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json::<Extension>()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))
	}

	async fn install_extension(
		client: &Client,
		extension: &Extension,
		profile: &Profile,
	) -> Result<()> {
		let version = extension.current_version.file.id;
		let guid = extension.guid.clone();
		let name = extension.get_name();

		let response = client
			.get(format!("{DOWNLOAD_URL}/{version}"))
			.send()
			.await
			.or(Err(Error::Install(name.clone())))?;

		let extensions_folder = profile.path.join("extensions");

		if !extensions_folder.exists() {
			tokio::fs::create_dir(&extensions_folder).await?;
		}

		let path = format!("{}.xpi", extensions_folder.join(guid).display());
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
	pub async fn try_configure_from(
		val: Vec<String>,
		configuration: crate::cli::Configuration,
	) -> Result<Self> {
		let configuration = Configuration::try_configure_from(configuration).await?;
		let client = Client::new();
		let mut extensions = Vec::new();

		for extension in val {
			match Self::find_extension(&client, &extension).await {
				Ok(ext) => extensions.push(ext),
				Err(err) => return Err(err),
			};
		}

		Ok(Self {
			extensions,
			configuration,
			client,
		})
	}
}

impl Runnable for Install {
	async fn try_run(&self) -> Result<()> {
		for ext in &self.extensions {
			let name = ext.get_name();
			println!("{}: {}", "Installing extension".bold().bright_blue(), name);
			println!("{}", ext);

			match Self::install_extension(&self.client, ext, &self.configuration.profile).await {
				Ok(_) => {
					Manifest::add_extension_to_database(&self.configuration.profile, ext).await?
				}

				Err(err) => {
					eprintln!("{}: {}", "Error".bold().red(), err);
					eprintln!("{}: {}", "Error installing extension".bold().red(), name);
				}
			}
		}

		Ok(())
	}
}
