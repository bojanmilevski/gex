use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct InstallTelemetryInfo {
	pub method: Option<String>,
	pub source: Option<String>,
}

impl InstallTelemetryInfo {
	pub fn new() -> Self {
		Self { source: Some(String::from("app-profile")), method: Some(String::from("sideload")) }
	}
}
