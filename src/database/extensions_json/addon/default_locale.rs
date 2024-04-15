use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct DefaultLocale {
	contributors: Option<String>,
	creator: Option<String>,
	description: Option<String>,
	developers: Option<String>,
	#[serde(rename = "homepageURL")]
	homepage_url: Option<Url>,
	name: Option<String>,
	translators: Option<String>,
}

impl TryFrom<&Manifest> for DefaultLocale {
	type Error = Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let default_locale = Self {
			contributors: None,
			creator: manifest.author.clone(),
			description: None,
			developers: None,
			homepage_url: None,
			name: Some(manifest.name.clone()),
			translators: None,
		};

		Ok(default_locale)
	}
}
