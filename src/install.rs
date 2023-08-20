use crate::config;
use crate::errors;
use crate::query;

use config::DOWNLOAD_URL;
use errors::InstallError;
use query::Extension;

pub async fn install_extension(extension: &Extension, download_path: &str) -> Result<(), InstallError> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let request = reqwest::get(format!("{}/{}", DOWNLOAD_URL, &ext_ver)).await?;

	if !request.status().is_success() {
		return Err(InstallError::InstallUnsuccessfull);
	}

	let path = format!("{}/{}.xpi", &download_path, &ext_guid);
	let mut file = std::fs::File::create(&path)?;
	std::io::copy(&mut request.bytes().await?.as_ref(), &mut file)?;

	Ok(())
}
