use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ContributionsUrl {
	UrlOutgoing(HashMap<String, Url>),
	Empty(String), // is always an empty string
}
