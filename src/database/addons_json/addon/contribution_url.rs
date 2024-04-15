use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContributionUrl {
	Url(Url),
	Empty(String),
}
