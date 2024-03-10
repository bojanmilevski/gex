use super::message::Message;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct LocaleFile {
	pub extension_description: Option<Message>,
}
