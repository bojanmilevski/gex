use serde::Deserialize;

#[derive(Deserialize)]
pub struct Promoted {
	apps: Vec<String>,
	category: String,
}
