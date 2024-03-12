use crate::database::manifest::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default)]
pub struct Permissions {
	pub permissions: Vec<String>,
	pub origins: Vec<String>,
}

impl TryFrom<&Manifest> for Permissions {
	type Error = Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let mut permissions = manifest.permissions.clone();
		permissions.retain(|p| !p.starts_with('<'));
		let origins = manifest
			.permissions
			.clone()
			.into_iter()
			.filter(|o| o.starts_with('<'))
			.collect();

		Ok(Self {
			permissions,
			origins,
		})
	}
}
