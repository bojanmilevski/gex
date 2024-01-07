use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Ini error.")]
	Ini(#[from] ini::Error),

	#[error("Request error.")]
	Reqwest(#[from] reqwest::Error),

	#[error("IO Error.")]
	IO(#[from] std::io::Error),

	#[error("serde_json error.")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Can not find HOME enviornment variable.")]
	Home,

	#[error("Browser {0} path not found.")]
	BrowserPathNotFound(String),

	#[error("Browser {0} not supported.")]
	BrowserNotSupported(String),

	#[error("Profile {0} not found.")]
	ProfileNotFound(String),

	#[error("Extension {0} not found.")]
	ExtensionNotFound(String),

	#[error("Error while installing extension {0}")]
	Install(String),

	#[error("Content length not received for extension {0}")]
	ContentLength(String),

	#[error("Querying extension {0} failed")]
	Query(String),
}
