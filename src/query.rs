use crate::config::QUERY_URL;
use crate::errors::QueryError;
use crate::extension::Extension;
use crate::extension::QueryResult;
use rayon::prelude::*;

pub async fn query_extension(ext_slug: &str) -> Result<Extension, QueryError> {
	let query_request: QueryResult = reqwest::Client::new()
		.get(format!("{}{}", QUERY_URL, ext_slug))
		.send()
		.await?
		.json()
		.await?;

	query_request
		.results
		.par_iter()
		.find_any(|ext| ext.slug == ext_slug)
		.ok_or(QueryError::ExtensionNotFound(ext_slug))
		.cloned()
}
