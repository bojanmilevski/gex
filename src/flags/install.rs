use super::configurable::Configurable;
use super::flags::Flags;
use super::profile::Profile;
use super::runnable::Runnable;
use crate::api::urls::DOWNLOAD_URL;
use crate::api::urls::EXTENSION_URL;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::progress_bar::Bar;
use colored::Colorize;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

pub struct Install {
	pub extensions: Vec<Extension>,
}

impl Install {
	async fn find_extension(slug: &str) -> Result<Extension> {
		Ok(reqwest::Client::new()
			.get(format!("{}/{}", EXTENSION_URL, slug))
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json::<Extension>()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))?)
	}

	async fn install_extension(extension: &Extension, profile: &Profile) -> Result<()> {
		let version = &extension.current_version.file.id;
		let guid = &extension.guid;
		let name = extension.get_name();
		let url = format!("{}/{}", DOWNLOAD_URL, version);

		let response = Client::new()
			.get(url)
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
		let mut bar = Bar::new(total_size)?;

		let mut bytes_stream = response.bytes_stream();

		while let Some(item) = bytes_stream.next().await {
			let chunk = item?;
			file.write_all(&chunk).await?;
			bar.update(chunk.len());
		}

		Ok(())
	}
}

impl Configurable for Install {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		let mut extensions = Vec::new();

		for extension in cli.operation.extensions.clone() {
			match Self::find_extension(&extension).await {
				Ok(ext) => extensions.push(ext),
				Err(err) => return Err(err),
			};
		}

		Ok(Self { extensions })
	}
}

impl Runnable for Install {
	async fn try_run(&self, flags: &Flags) -> Result<()> {
		for ext in &self.extensions {
			let name = ext.get_name();
			println!("{}: {}", "Installing extension".bold().bright_blue(), name);
			println!("{}", ext);

			if let Err(err) = Self::install_extension(&ext, &flags.profile).await {
				eprintln!("{}: {}", "Error".bold().red(), err);
				eprintln!("{}: {}", "Error installing extension".bold().red(), name);
			} else {
				crate::database::database::add_extension_to_database(&flags, &ext).await?;
			};
		}

		Ok(())
	}
}
