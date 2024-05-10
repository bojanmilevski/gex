use super::request::Request;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebRequest {
	on_before_request: Vec<(Request, Vec<String>)>,
	on_before_send_headers: Vec<(Request, Vec<String>)>,
	on_headers_received: Vec<(Request, Vec<String>)>,
}
