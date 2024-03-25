use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetApplication {
	pub id: String,
	pub max_version: Option<String>,
	pub min_version: Option<String>,
}

impl TryFrom<&Manifest> for TargetApplication {
	type Error = Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let id = String::from("toolkit@mozilla.org");
		let min_version = manifest
			.browser_specific_settings
			.gecko
			.strict_min_version
			.clone();
		let max_version = manifest
			.browser_specific_settings
			.gecko
			.strict_max_version
			.clone();

		Ok(Self { id, min_version, max_version })
	}
}
