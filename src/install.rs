use crate::config::DOWNLOAD_URL;
use crate::errors::InstallError;
use crate::extension::Extension;
use crate::profile::Profile;

pub async fn install_extension(extension: &Extension, profile: &Profile) -> Result<(), InstallError> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let ext_url = format!("{}/{}", DOWNLOAD_URL, &ext_ver);
	let request = reqwest::get(&ext_url).await?;

	if !request.status().is_success() {
		return Err(InstallError::InstallUnsuccessfull);
	}

	let mut path_str = profile.path.join(&ext_guid);
	path_str.set_extension("xpi");
	let mut ext_file = std::fs::File::create(&path_str)?;
	std::io::copy(&mut request.bytes().await?.as_ref(), &mut ext_file)?;

	Ok(())
}
