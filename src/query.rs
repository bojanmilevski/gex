use crate::errors::Error;
use crate::errors::Result;
use crate::extension::Extension;
use crate::extension::ExtensionsList;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const QUERY_URL: &str = "https://addons.mozilla.org/api/v5/addons/search/?q=";

async fn send_query(ext_slug: &str) -> Result<ExtensionsList> {
	let url = format!("{}{}", QUERY_URL, &ext_slug);
	let response = reqwest::Client::new().get(&url).send().await?;

	if !response.status().is_success() {
		return Err(Error::Send);
	}

	let json = response.json().await?;

	Ok(json)
}

pub async fn query_extension(ext_slug: &str) -> Result<Extension> {
	send_query(&ext_slug)
		.await?
		.extensions
		.par_iter()
		.find_any(|ext| ext.slug == ext_slug)
		.ok_or(Error::ExtensionNotFound)
		.cloned()
}

pub async fn query_extensions(ext_slug: &str) -> Result<Vec<Extension>> {
	Ok(send_query(&ext_slug).await?.extensions)
}
