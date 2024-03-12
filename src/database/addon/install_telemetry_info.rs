use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct InstallTelemetryInfo {
    pub source: Option<String>,
    pub method: Option<String>,
}

impl InstallTelemetryInfo {
    pub fn new() -> Self {
        Self {
            source: Some("app-profile".to_owned()),
            method: Some("sideload".to_owned()),
        }
    }
}
