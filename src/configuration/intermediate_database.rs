use crate::database::database::Database;
use crate::errors::Result;
use crate::operation::install::Install;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct IntermediateDatabase {
	pub map: HashMap<String, String>,
	pub slugs: Vec<String>,
	pub guids: Vec<String>,
}

impl IntermediateDatabase {
	pub async fn try_configure_from(database: &Database) -> Result<Self> {
		let guids = database
			.addons
			.iter()
			.map(|addon| addon.id.clone())
			.collect::<Vec<_>>();

		// FIX: run code below only once for the duration of the program
		// it also runs in browser.rs
		let home = home::home_dir().unwrap();
		let cache_path = home.join(".cache").join("gex");

		if !cache_path.exists() {
			tokio::fs::create_dir_all(&cache_path).await?;
		}

		let db_path = cache_path.join("database.json");

		if !db_path.exists() {
			tokio::fs::File::create(&db_path).await?;
		}

		let file = std::fs::File::open(&db_path)?;
		let mut reader = BufReader::new(file);
		let client = Client::new();
		let mut map: HashMap<String, String>;

		if reader.fill_buf()?.is_empty() {
			map = HashMap::new();
			for guid in &guids {
				let ext = Install::find_extension(&client, guid).await?;
				map.insert(guid.to_string(), ext.slug.clone());
			}
		} else {
			map = serde_json::from_reader(reader)?;

			let missing: Vec<&String> = guids
				.iter()
				.filter(|guid| map.get(*guid).is_none() || map.get(*guid).unwrap().is_empty())
				.collect();

			for guid in &missing {
				let ext = Install::find_extension(&client, guid).await?;
				map.insert(guid.to_string(), ext.slug.clone());
			}
		}

		let slugs = map.values().cloned().collect();
		let content = serde_json::to_string(&map)?;
		tokio::fs::write(db_path, content).await?;

		Ok(Self { map, guids, slugs })
	}
}
