use crate::api::DOWNLOAD_URL;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::flags::profile::Profile;
use crate::progress_bar::Bar;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

pub async fn install_extension(extension: Extension, profile: Profile) -> Result<()> {
	let version = extension.current_version.file.id;
	let guid = extension.guid;
	let name = extension.name.clone().name.unwrap_or("EMPTY".to_string());
	let url = format!("{}/{}", DOWNLOAD_URL, version);

	let response = Client::new()
		.get(url)
		.send()
		.await
		.or(Err(Error::Install(name.clone())))?;

	let extensions_folder = profile.path.join("extensions");

	if !extensions_folder.exists() {
		tokio::fs::create_dir(&extensions_folder).await?;
	}

	let file_path = format!("{}.xpi", extensions_folder.join(guid).display());
	let mut file = tokio::fs::File::create(file_path).await?;

	let total_size = response
		.content_length()
		.ok_or(Error::ContentLength(name))?;
	let mut bar = Bar::new(total_size)?;

	let mut bytes_stream = response.bytes_stream();

	while let Some(item) = bytes_stream.next().await {
		let chunk = item?;
		file.write_all(&chunk).await?;
		bar.update(chunk.len());
	}

	Ok(())
}
