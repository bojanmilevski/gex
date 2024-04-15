use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize)]
pub struct Preview {
	id: u64,
	caption: Option<HashMap<String, String>>,
	image_size: [u64; 2],
	image_url: Url,
	position: u64,
	tuhmbnail_size: Option<[u64; 2]>,
	thumbnail_url: Url,
}
