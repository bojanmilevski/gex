use super::gecko::Gecko;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Applications {
	pub gecko: Gecko,
}
