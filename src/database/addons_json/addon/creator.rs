use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Creator {
	name: String,
	url: Url,
}

impl From<&Url> for Creator {
	fn from(url: &Url) -> Self {
		Self { name: String::new(), url: url.clone() }
	}
}
