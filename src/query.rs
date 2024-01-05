use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::extension::extensions_list::ExtensionsList;

const QUERY_URL: &str = "https://addons.mozilla.org/api/v5/addons/search";

async fn send_query(ext_slug: &str) -> Result<ExtensionsList> {
	let response = reqwest::Client::new()
		.get(QUERY_URL)
		.query(&[("", ext_slug)])
		.send()
		.await?; // TODO: change with .or(Err(Error::QueryError))?;

	if !response.status().is_success() {
		return Err(Error::Send);
	}

	Ok(response.json().await?)
}

pub async fn query_extension(ext_slug: &str) -> Result<Extension> {
	send_query(&ext_slug)
		.await?
		.extensions
		.iter()
		.find(|ext| ext.slug == ext_slug)
		.ok_or(Error::ExtensionNotFound(ext_slug.to_owned()))
		.cloned()
}

pub async fn query_extensions(ext_slug: &str) -> Result<Vec<Extension>> {
	Ok(send_query(&ext_slug).await?.extensions)
}
