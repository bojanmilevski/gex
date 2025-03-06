use crate::cli::CliConfiguration;
use crate::database::database::Database;
use crate::traits::Runnable;
use anyhow::Result;

pub struct List {
	slugs: Vec<String>,
}

// FIX: init trait
impl List {
	pub async fn try_init(cli_configuration: CliConfiguration) -> Result<Self> {
		let database = Database::try_from(cli_configuration)?;
		let slugs = database.get_slugs();

		Ok(Self { slugs })
	}
}

impl Runnable for List {
	async fn try_run(&mut self) -> Result<()> {
		self.slugs.iter().for_each(|slug| println!("{}", slug));

		Ok(())
	}
}
