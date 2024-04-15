use super::web_request::WebRequest;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentListeners {
	web_request: WebRequest,
}
