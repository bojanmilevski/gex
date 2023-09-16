use crate::config;
use crate::errors;
use crate::extension;
use config::DOWNLOAD_URL;
use errors::InstallError;
use extension::Extension;
use std::path::PathBuf;

pub async fn install_extension(extension: &Extension, download_path: &PathBuf) -> Result<(), InstallError> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let ext_url = format!("{}/{}", DOWNLOAD_URL, &ext_ver);
	let request = reqwest::get(&ext_url).await?;

	if !request.status().is_success() {
		return Err(InstallError::InstallUnsuccessfull);
	}

	let path_str = format!("{}/{}.xpi", &download_path.display(), &ext_guid);
	let mut ext_file = std::fs::File::create(&path_str)?;
	std::io::copy(&mut request.bytes().await?.as_ref(), &mut ext_file)?;

	Ok(())
}
