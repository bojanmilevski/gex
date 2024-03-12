use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationState {
	pub valid_not_after: u64,
	pub valid_not_before: u64,
	pub states: Vec<String>,
}

impl RecommendationState {
	pub fn new() -> Self {
		Self {
			valid_not_after: 0,
			valid_not_before: 0,
			states: vec!["recommended-android".to_owned(), "recommended".to_owned()],
		}
	}
}
