use crate::config::QUERY_URL;
use crate::errors::QueryError;
use crate::extension::Extension;
use crate::extension::QueryResult;

use rayon::prelude::*;
use reqwest::Response;

async fn get_query(ext_slug: String) -> Result<Response, QueryError> {
	let url = format!("{}{}", QUERY_URL, ext_slug);

	let query_request = reqwest::Client::new().get(&url).send().await?;

	if !query_request.status().is_success() {
		return Err(QueryError::Send);
	}

	Ok(query_request)
}

pub async fn query_extensions_list(ext_slug: String) -> Result<Vec<Extension>, QueryError> {
	let query = get_query(ext_slug).await?;
	let json: QueryResult = query.json().await?;
    let list: Vec<Extension> = json.results.par_iter().map(|e| e.clone()).collect();
	Ok(list)
}

pub async fn query_extension(ext_slug: String) -> Result<Extension, QueryError> {
	let query = get_query(ext_slug.clone()).await?;
	let json: QueryResult = query.json().await?;

	json.results
		.par_iter()
		.find_any(|ext| ext.slug == ext_slug)
		.ok_or(QueryError::ExtensionNotFound)
		.cloned()
}
