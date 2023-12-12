use crate::errors::Error;
use crate::errors::Result;
use crate::extension::Extension;
use crate::flags::Profile;

const DOWNLOAD_URL: &str = "https://addons.mozilla.org/firefox/downloads/file";

pub async fn install_extension(extension: &Extension, profile: &Profile) -> Result<()> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let url = format!("{}/{}", DOWNLOAD_URL, &ext_ver);
	let client = reqwest::Client::new();
	let content_len = client.head(&url).send().await?.content_length();

	if let None = content_len {
		return Err(Error::InstallUnsuccessfull);
	}

	let request = client.get(&url).send().await?;

	if !request.status().is_success() {
		return Err(Error::InstallUnsuccessfull);
	}

	let mut path = profile.path.join(&ext_guid);
	path.set_extension("xpi");
	let mut file = tokio::fs::File::create(&path).await?;
	let bytes = request.bytes().await?;
	tokio::io::copy(&mut bytes.as_ref(), &mut file).await?;

	Ok(())
}
