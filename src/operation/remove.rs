use crate::cli::CliConfiguration;
use crate::database::database::Database;
use crate::traits::Runnable;
use anyhow::Result;

pub struct Remove {
	slugs: Vec<String>,
	database: Database,
}

// FIX: init trait
impl Remove {
	pub async fn try_init(slugs: Vec<String>, cli_configuration: CliConfiguration) -> Result<Self> {
		// let database = Database::try_from(cli_configuration)?;
		// let slugs = database
		// 	.get_addons(Some(slugs))?
		// 	.iter()
		// 	.map(|(slug, _, _)| slug.clone())
		// 	.collect();

		// Ok(Self { slugs, database })

		todo!()
	}
}

impl Runnable for Remove {
	async fn try_run(&mut self) -> Result<()> {
		// let slugs: Vec<&str> = self.slugs.iter().map(|slug| slug.as_ref()).collect(); // FIX: void having to do this
		// let removed = self.database.remove_from_database(&slugs)?;
		// self.database.remove_from_disk()?;
		// self.database.write_to_disk()?;

		Ok(())
	}
}
