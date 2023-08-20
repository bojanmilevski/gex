use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallError {
	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("Profile path error.")]
	Profile(#[from] ProfileError),

	#[error("Creating file error.")]
	CreateFile(#[from] std::io::Error),

	#[error("Install was unsuccessfull.")]
	InstallUnsuccessfull,
}

#[derive(Debug, Error)]
pub enum ProfileError {
	#[error("Can not find $HOME enviornment variable.")]
	HomeVar(#[from] std::env::VarError),

	#[error("Ini error.")]
	Ini(#[from] ini::Error),

	#[error("Profile not found.")]
	ProfileNotFound,

	#[error("Browser path not found.")]
	BrowserPathNotFound,

	#[error("Browser not supported.")]
	BrowserNotSupported,
}

#[derive(Debug, Error)]
pub enum QueryError<'a> {
	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("Extension {0} not found.")]
	ExtensionNotFound(&'a str),
}
