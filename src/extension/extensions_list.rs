use crate::extension::extension::Extension;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionsList {
	#[serde(rename = "results")]
	pub extensions: Vec<Extension>,
}
