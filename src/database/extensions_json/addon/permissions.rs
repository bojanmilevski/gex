use crate::addon::addon::Addon;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Permissions {
	pub origins: Vec<String>,
	pub permissions: Vec<String>,
}

impl TryFrom<&Addon> for Permissions {
	type Error = Error;

	fn try_from(addon: &Addon) -> Result<Self> {
		let permissions = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| !addon.contains(".com"))
			.cloned()
			.collect();

		let origins = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| addon.contains(".com"))
			.cloned()
			.collect();

		Ok(Self { origins, permissions })
	}
}
