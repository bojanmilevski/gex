use super::persistent_listeners::PersistentListeners;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupData {
	persistent_listeners: Option<PersistentListeners>,
}
