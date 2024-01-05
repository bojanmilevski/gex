use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::flags::profile::Profile;
use crate::progress_bar::Bar;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

const DOWNLOAD_URL: &str = "https://addons.mozilla.org/firefox/downloads/file";

pub async fn install_extension(extension: &Extension, profile: &Profile) -> Result<()> {
	let guid = &extension.guid;
	let version = &extension.current_version.file.id;
	let url = format!("{}/{}", DOWNLOAD_URL, version);
	let response = Client::new().get(url).send().await?; // TODO: replace with .or(Err(Error::InstallUnsuccessfull))?;

	/* if !response.status().is_success() {
		return Err(Error::InstallUnsuccessfull);
	} */

	let total_size = response.content_length().unwrap();
	// .ok_or(Error::InstallUnsuccessfull)?;

	let mut stream = response.bytes_stream();
	let file_path = format!("{}.xpi", profile.path.join("extensions").join(&guid).display());
	let mut file = tokio::fs::File::create(file_path).await?;
	let mut bar = Bar::new(total_size)?;

	while let Some(item) = stream.next().await {
		let chunk = item?;
		file.write_all(&chunk).await?;
		bar.update(chunk.len());
	}

	Ok(())
}
