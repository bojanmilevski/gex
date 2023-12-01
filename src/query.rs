use crate::errors::QueryError;
use crate::extension::Extension;
use crate::extension::QueryResult;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const QUERY_URL: &str = "https://addons.mozilla.org/api/v5/addons/search/?q=";

async fn send_query(ext_slug: &String) -> Result<QueryResult, QueryError> {
	let url = format!("{}{}", QUERY_URL, &ext_slug);
	let query_request = reqwest::Client::new().get(&url);
	let query_response = query_request.send().await?;

	if !query_response.status().is_success() {
		return Err(QueryError::Send);
	}

	let json: QueryResult = query_response.json().await?;

	Ok(json)
}

pub async fn query_extension(ext_slug: &String) -> Result<Extension, QueryError> {
	let json = send_query(&ext_slug).await?;

	json.extensions
		.par_iter()
		.find_any(|ext| &ext.slug == ext_slug)
		.ok_or(QueryError::ExtensionNotFound)
		.cloned()
}

pub async fn query_extensions(ext_slug: &String) -> Result<Vec<Extension>, QueryError> {
	let json = send_query(&ext_slug).await?;
	Ok(json.extensions)
}
