use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screenshot {
	url: Url,
	width: u32,
	height: u32,
	#[serde(rename = "thumbnailURL")]
	thumbnail_url: Url,
	thumbnail_width: u32,
	thumbnail_height: u32,
	caption: Option<String>,
}
