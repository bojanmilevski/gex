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
