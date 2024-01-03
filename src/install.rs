use crate::errors::Error;
use crate::errors::Result;
use crate::extension::Extension;
use crate::flags::Profile;
use reqwest::Client;

const DOWNLOAD_URL: &str = "https://addons.mozilla.org/firefox/downloads/file";

pub async fn install_extension(extension: &Extension, profile: &Profile) -> Result<()> {
	let guid = &extension.guid;
	let version = &extension.current_version.file.id;
	let url = format!("{}/{}", DOWNLOAD_URL, &version);
	let response = Client::new().get(&url).send().await?;

	if !response.status().is_success() {
		return Err(Error::InstallUnsuccessfull);
	}

	let path = format!("{}.xpi", profile.path.join("extensions").join(&guid).display());
	let mut file = tokio::fs::File::create(&path).await?;
	let bytes = response.bytes().await?;
	tokio::io::copy(&mut bytes.as_ref(), &mut file).await?;

	Ok(())
}
