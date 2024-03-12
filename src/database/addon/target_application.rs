use crate::database::manifest::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetApplication {
	pub id: String,
	pub min_version: Option<String>,
	pub max_version: Option<String>,
}

impl TryFrom<&Manifest> for TargetApplication {
	type Error = Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		Ok(Self {
			id: String::from("toolkit@mozilla.org"),
			min_version: manifest
				.browser_specific_settings
				.gecko
				.strict_min_version
				.clone(),
			max_version: manifest
				.browser_specific_settings
				.gecko
				.strict_max_version
				.clone(),
		})
	}
}
