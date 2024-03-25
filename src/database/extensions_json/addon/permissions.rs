use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default)]
pub struct Permissions {
	pub origins: Vec<String>,
	pub permissions: Vec<String>,
}

impl TryFrom<&Manifest> for Permissions {
	type Error = Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let permissions = manifest
			.permissions
			.clone()
			.unwrap()
			.into_iter()
			.filter(|p| !p.starts_with('<'))
			.collect();

		let origins = manifest
			.permissions
			.clone()
			.unwrap()
			.into_iter()
			.filter(|o| o.starts_with('<'))
			.collect();

		Ok(Self { origins, permissions })
	}
}
