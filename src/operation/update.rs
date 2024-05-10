use super::install::Install;
use crate::addon::addon::Addon;
use crate::cli::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::traits::runnable::Runnable;
use anyhow::Result;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use reqwest::Client;

pub struct Update {
	install: Install,
}

impl Update {
	async fn filter_addons(
		slugs: Option<Vec<String>>,
		configuration: &Configuration,
		client: &Client,
	) -> Result<Vec<Addon>> {
		let addons_map = configuration.database.get_addons(slugs)?;
		let addons = futures_util::stream::iter(addons_map)
			.then(|(slug, _, version)| async move {
				match Install::find_addon(client, &slug).await {
					Ok(new) if new.version() > version => Some(Ok(new)),
					Ok(_) => None,
					Err(err) => Some(Err(err)),
				}
			})
			.filter_map(|addon| async { addon })
			.try_collect()
			.await?;

		Ok(addons)
	}
}

// FIX: configurable trait
impl Update {
	pub async fn try_configure_from(slugs: Option<Vec<String>>, cli_configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(cli_configuration)?;
		let client = Client::new();
		let addons = Self::filter_addons(slugs, &configuration, &client).await?;
		let install = Install { addons, client, configuration };

		Ok(Self { install })
	}
}

impl Runnable for Update {
	async fn try_run(&mut self) -> Result<()> {
		self.install.try_run().await?;
		Ok(())
	}
}
