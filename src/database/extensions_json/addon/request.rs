use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
	incognito: Option<bool>,
	tab_id: Option<u32>,
	types: Vec<String>,
	urls: Vec<String>, // FIX: Url
	window_id: Option<u32>,
	#[serde(default)]
	actions: Vec<String>,
}
