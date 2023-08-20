use crate::config::QUERY_URL;
use crate::errors::QueryError;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
	pub results: Vec<Extension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
	pub id: i32,
	pub slug: String,
	pub guid: String,
	pub current_version: CurrentVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentVersion {
	pub file: FileCurrentVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCurrentVersion {
	pub id: i32,
}

pub async fn query_extension(ext_slug: &str) -> Result<Extension, QueryError> {
	let query_request: QueryResult = reqwest::Client::new()
		.get(format!("{}{}", QUERY_URL, ext_slug))
		.send()
		.await?
		.json()
		.await?;

	query_request
		.results
		.iter()
		.find(|&ext| &ext.slug == &ext_slug)
		.ok_or(QueryError::ExtensionNotFound(ext_slug))
		.cloned()
}
