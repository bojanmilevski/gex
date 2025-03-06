use super::install::Install;
use crate::addon::Addon;
use crate::cli::CliConfiguration;
use crate::database::database::Database;
use crate::traits::Runnable;
use anyhow::Result;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use reqwest::Client;

pub struct Update {
	install: Install,
}

impl Update {
	async fn filter_addons(slugs: Option<Vec<String>>, database: &Database, client: &Client) -> Result<Vec<Addon>> {
		// let addons_map = database.get_addons(slugs)?;
		// let addons = futures_util::stream::iter(addons_map)
		// 	.then(|(slug, _, version)| async move {
		// 		match Install::find_addon(client, &slug).await {
		// 			Ok(new) if new.version() > version => Some(Ok(new)),
		// 			Ok(_) => None,
		// 			Err(err) => Some(Err(err)),
		// 		}
		// 	})
		// 	.filter_map(|addon| async { addon })
		// 	.try_collect()
		// 	.await?;

		// Ok(addons)
		todo!()
	}
}

// FIX: init trait
impl Update {
	pub async fn try_init(slugs: Option<Vec<String>>, cli_configuration: CliConfiguration) -> Result<Self> {
		let database = Database::try_from(cli_configuration)?;
		let client = Client::new();
		let addons = Self::filter_addons(slugs, &database, &client).await?;
		let install = Install { addons, client, database };

		Ok(Self { install })
	}
}

impl Runnable for Update {
	async fn try_run(&mut self) -> Result<()> {
		self.install.try_run().await?;
		Ok(())
	}
}
