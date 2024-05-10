use crate::database::manifests::manifest::Manifest;
use anyhow::Result;
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
	type Error = anyhow::Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let id = String::from("toolkit@mozilla.org");
		let min_version = None;
		let max_version = None;

		Ok(Self { id, min_version, max_version })
	}
}
