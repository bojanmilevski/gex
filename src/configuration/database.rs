use crate::addon::addons::Addons;
use crate::errors::Result;
use crate::operation::install::Install;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Database {
	addons: Addons,
	pub db: HashMap<String, String>,
	pub slugs: Vec<String>,
	pub guids: Vec<String>,
}

impl Database {
	pub async fn try_configure_from(path: &PathBuf) -> Result<Self> {
		// TODO: check if already present
		let addons = Addons::try_from(path)?;

		let guids = addons
			.addons
			.iter()
			.map(|addon| addon.id.clone())
			.collect::<Vec<_>>();

		let client = Client::new();
		let mut db = HashMap::new();

		for guid in &guids {
			let ext = Install::find_extension(&client, guid).await?;
			db.insert(guid.clone(), ext.slug.clone());
		}

		let slugs = db.values().map(|ext| ext.clone()).collect();

		let home = home::home_dir().unwrap();
		let gex_path = home.join(".cache").join("gex");

		if !gex_path.exists() {
			tokio::fs::create_dir(&gex_path).await?;
		}

		let database_json_path = gex_path.join("database.json");

		let content = serde_json::to_string(&db)?;
		tokio::fs::write(&database_json_path, content).await?;

		Ok(Self { addons, db, guids, slugs })
	}
}
