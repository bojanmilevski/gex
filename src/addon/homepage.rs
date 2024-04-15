use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Homepage {
	url: HashMap<String, Option<String>>, // FIX: Url
	outgoing: HashMap<String, Option<String>>,
}
