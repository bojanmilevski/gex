use crate::config::QUERY_URL;

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCurrentVersion {
	pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentVersion {
	pub file: FileCurrentVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
	pub id: i32,
	pub slug: String,
	pub guid: String,
	pub current_version: CurrentVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
	pub results: Vec<Extension>,
}

pub async fn query_extension(extension: &str) -> Result<QueryResult, reqwest::Error> {
	let query_request = reqwest::Client::new().get(format!("{}{}", QUERY_URL, extension)).send().await?.json().await?;
	Ok(query_request)
}
