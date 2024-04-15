use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Addon {0} not found.")]
	AddonNotFound(String),

	#[error("Browser {0} not supported.")]
	BrowserNotSupported(String),

	#[error("Browser {0} path not found.")]
	BrowserPathNotFound(String),

	#[error("Can not find HOME enviornment variable.")]
	Home,

	#[error("IO Error.")]
	IO(#[from] std::io::Error),

	#[error("Ini error.")]
	Ini(#[from] ini::Error),

	#[error("Error installing addon {0}")]
	Install(String),

	#[error("URL parse error for addon {0}")]
	URLParse(#[from] url::ParseError),

	#[error("Profile {0} not found.")]
	ProfileNotFound(String),

	#[error("Querying addon {0} failed")]
	Query(String),

	#[error("Request error.")]
	Reqwest(#[from] reqwest::Error),

	#[error("serde_json error.")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Plugins not installed: ")]
	Update(String),
}
