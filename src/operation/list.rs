use crate::cli::Configuration as CliConfiguration;
use crate::configuration::configuration::Configuration;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub struct List {
	list: Vec<String>,
}

impl List {
	pub async fn try_configure_from(configuration: CliConfiguration) -> Result<Self> {
		let configuration = Configuration::try_from(configuration)?;
		let list = configuration.database.get();

		Ok(Self { list })
	}
}

impl Runnable for List {
	async fn try_run(&mut self) -> Result<()> {
		self.list.iter().for_each(|ext| println!("{}", ext));
		Ok(())
	}
}
