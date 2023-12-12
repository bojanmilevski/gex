use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Ini error.")]
	Ini(#[from] ini::Error),

	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("IO Error.")]
	IO(#[from] std::io::Error),

	#[error("Can not find HOME enviornment variable.")]
	HomeVar,

	#[error("Browser path not found.")]
	BrowserPathNotFound,

	#[error("Browser not supported.")]
	BrowserNotSupported,

	#[error("Install was unsuccessfull.")]
	InstallUnsuccessfull,

	#[error("Profile not found.")]
	ProfileNotFound,

	#[error("Request was not sent successfully.")]
	Send,

	#[error("Extension not found.")]
	ExtensionNotFound,
}
