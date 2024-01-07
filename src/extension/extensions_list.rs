use super::extension::Extension;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExtensionsList {
	#[serde(rename = "results")]
	pub extensions: Vec<Extension>,
}
