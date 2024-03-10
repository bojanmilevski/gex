use crate::configuration::configuration::Configuration;
use crate::errors::Result;
use crate::runnable::Runnable;

pub struct List {
	list: Vec<String>,
	_configuration: Configuration,
}

impl List {
	pub async fn try_configure_from(config: crate::cli::Configuration) -> Result<Self> {
		let _configuration = Configuration::try_configure_from(config).await?;
		let list = _configuration.intermediate_database.slugs.clone();
		Ok(Self {
			_configuration,
			list,
		})
	}
}

impl Runnable for List {
	async fn try_run(&self) -> Result<()> {
		self.list.iter().for_each(|ext| println!("{}", ext));
		Ok(())
	}
}
