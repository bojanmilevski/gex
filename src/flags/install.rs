use super::configurable::Configurable;
use crate::api_url::API_URL;
use crate::cli::Cli;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Install {
	pub extensions: Vec<Extension>,
}

impl Install {
	async fn find_extension(slug: &str) -> Result<Extension> {
		Ok(reqwest::Client::new()
			.get(format!("{}/{}/{}", API_URL, "addon", slug))
			.send()
			.await
			.or(Err(Error::Query(slug.to_owned())))?
			.json()
			.await
			.or(Err(Error::ExtensionNotFound(slug.to_owned())))?)
	}
}

impl Configurable for Install {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		let mut extensions = Vec::new();

		for extension in &cli.operation.extensions {
			match Self::find_extension(&extension).await {
				Ok(ext) => extensions.push(ext),
				Err(err) => return Err(err),
			};
		}

		Ok(Self { extensions })
	}
}
