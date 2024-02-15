use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct InstallTelemetryInfo {
	pub source: String,
	pub method: String,
}

impl InstallTelemetryInfo {
	pub fn new() -> Self {
		Self { source: "app-profile".to_owned(), method: "sideload".to_owned() }
	}
}
