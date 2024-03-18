use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct InstallTelemetryInfo {
	pub method: Option<String>,
	pub source: Option<String>,
}

impl InstallTelemetryInfo {
	pub fn new() -> Self {
		Self { source: Some("app-profile".to_owned()), method: Some("sideload".to_owned()) }
	}
}
