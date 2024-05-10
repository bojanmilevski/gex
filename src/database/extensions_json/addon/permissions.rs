use crate::addon::addon::Addon;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Permissions {
	pub origins: Vec<String>,
	pub permissions: Vec<String>,
}

impl TryFrom<&Addon> for Permissions {
	type Error = anyhow::Error;

	fn try_from(addon: &Addon) -> Result<Self> {
		let permissions = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| !addon.contains(".com")) // FIX: this is not right
			.cloned()
			.collect();

		let origins = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| addon.contains(".com")) // FIX: this is not right
			.cloned()
			.collect();

		Ok(Self { origins, permissions })
	}
}
