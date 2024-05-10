use crate::cli::CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::traits::runnable::Runnable;
use anyhow::Result;

pub struct List {
	slugs: Vec<String>,
}

// FIX: configurable trait
impl List {
	pub async fn try_configure_from(configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(configuration)?;
		let slugs = configuration.database.get_slugs();

		Ok(Self { slugs })
	}
}

impl Runnable for List {
	async fn try_run(&mut self) -> Result<()> {
		self.slugs.iter().for_each(|slug| println!("{}", slug));

		Ok(())
	}
}
