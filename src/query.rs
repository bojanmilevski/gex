use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::extension::extensions_list::ExtensionsList;

const QUERY_URL: &str = "https://addons.mozilla.org/api/v5/addons/search";

pub async fn query_extensions(ext_slug: &str) -> Result<ExtensionsList> {
	Ok(reqwest::Client::new()
		.get(QUERY_URL)
		.query(&[("q", ext_slug), ("page_size", "1000"), ("lang", "en-US"), ("sort", "users")])
		.send()
		.await
		.or(Err(Error::Placeholder("send_query".to_owned())))?
		.json()
		.await?)

	/* if !response.status().is_success() {
		return Err(Error::Send);
	}

	Ok(response.json().await?) */
}

pub async fn find_extension(ext_slug: &str) -> Result<Extension> {
	query_extensions(&ext_slug)
		.await?
		.extensions
		.iter()
		.find(|ext| ext.slug == ext_slug)
		.ok_or(Error::ExtensionNotFound(ext_slug.to_owned()))
		.cloned()
}
