use crate::errors::{Error, Result};
use crate::extension::extension::Extension;
use crate::flags::profile::Profile;
use crate::progress_bar::Bar;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

const DOWNLOAD_URL: &str = "https://addons.mozilla.org/firefox/downloads/file";

pub async fn install_extension(extension: Extension, profile: Profile) -> Result<()> {
	let url = format!("{}/{}", DOWNLOAD_URL, extension.current_version.file.id);
	let response = Client::new()
		.get(url)
		.send()
		.await
		.or(Err(Error::Placeholder("install_extension -> response".to_owned())))?;

	/* if !response.status().is_success() {
		return Err(Error::InstallUnsuccessfull);
	} */

	let file_path = format!(
		"{}.xpi",
		profile
			.path
			.join("extensions")
			.join(&extension.guid)
			.display()
	);
	let mut file = tokio::fs::File::create(file_path).await?;

	let total_size = response
		.content_length()
		.ok_or(Error::Placeholder("install_extension -> total_size".to_owned()))?;
	let mut bar = Bar::new(total_size)?;

	let mut bytes_stream = response.bytes_stream();

	while let Some(item) = bytes_stream.next().await {
		let chunk = item?;
		file.write_all(&chunk).await?;
		bar.update(chunk.len());
	}

	Ok(())
}
