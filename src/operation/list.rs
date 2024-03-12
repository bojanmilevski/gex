use crate::configuration::configuration::Configuration;
use crate::errors::Result;
use crate::traits::runnable::Runnable;

pub struct List {
	list: Vec<String>,
}

impl List {
	pub async fn try_configure_from(config: crate::cli::Configuration) -> Result<Self> {
		let configuration = Configuration::try_from(config)?;
		let list = configuration
			.addons_json_database
			.addons
			.iter()
			.map(|addon| addon.slug.clone())
			.collect::<Vec<_>>()
			.clone();

		Ok(Self { list })
	}
}

impl Runnable for List {
	async fn try_run(&self) -> Result<()> {
		self.list.iter().for_each(|ext| println!("{}", ext));
		Ok(())
	}
}
