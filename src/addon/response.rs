use super::addon::Addon;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Response {
	Addon(Addon),
	NotFound(NotFound),
	Authentication(Authentication),
}

#[derive(Deserialize)]
struct NotFound {
	detail: String,
}

#[derive(Deserialize)]
struct Authentication {
	detail: String,
	is_disabled_by_developer: bool,
	is_disabled_by_mozilla: bool,
}
