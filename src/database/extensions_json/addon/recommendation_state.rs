use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationState {
	pub states: Vec<String>,
	pub valid_not_after: u64,
	pub valid_not_before: u64,
}

impl RecommendationState {
	pub fn new() -> Self {
		Self {
			states: vec![String::from("recommended-android"), String::from("recommended")],
			valid_not_after: 0,
			valid_not_before: 0,
		}
	}
}
