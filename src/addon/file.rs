use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct File {
	pub id: u64,
	// created: DateTime<Utc>,
	// hash: String,
	// is_mozilla_signed_extension: bool,
	// size: u64,
	// status: String,
	pub url: Url,
	pub permissions: Vec<String>,
	pub optional_permissions: Vec<String>,
	// host_permissions: Vec<String>,
}
