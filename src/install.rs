use crate::config;
use crate::errors;
use crate::query;

use config::DOWNLOAD_URL;
use errors::InstallError;
use query::query_extension;
use query::Extension;

use std::fs::File;
use std::io::copy;

async fn install_extension(extension: &Extension, download_path: &String) -> Result<(), InstallError> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let request = reqwest::get(format!("{}/{}", DOWNLOAD_URL, &ext_ver)).await?;
	if request.status().is_success() {
		let ext_path = format!("{}/{}.xpi", &download_path, &ext_guid);
		let mut file = File::create(&ext_path)?;
		copy(&mut request.bytes().await?.as_ref(), &mut file)?;
	}

	Ok(())
}

pub async fn install_extensions(extensions: &[String], download_path: &String) -> Result<(), InstallError> {
	for ext in extensions {
		let query_result = query_extension(&ext);
		if let Some(extension) = query_result.await?.results.first() {
			install_extension(extension, download_path).await?;
		} else {
			eprintln!("Extension not found: {}", &ext);
		}
	}

	Ok(())
}
