use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResult {
	pub results: Vec<Extension>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Extension {
	pub id: i32,
	pub slug: String,
	pub guid: String,
	pub current_version: CurrentVersion,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentVersion {
	pub file: FileCurrentVersion,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileCurrentVersion {
	pub id: i32,
}
