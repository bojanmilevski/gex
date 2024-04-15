use serde::Deserialize;

#[derive(Deserialize)]
pub struct Compatibility {
	firefox: CompatibilityType,
	android: Option<CompatibilityType>,
}

#[derive(Deserialize)]
struct CompatibilityType {
	min: String,
	max: String,
}
