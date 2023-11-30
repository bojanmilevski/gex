use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
	#[error("Can not find $HOME enviornment variable.")]
	HomeVar(#[from] std::env::VarError),

	#[error("Browser path not found.")]
	PathNotFound,

	#[error("Browser not supported.")]
	NotSupported,
}

#[derive(Debug, Error)]
pub enum FlagsError {
	#[error("Profile error.")]
	Profile(#[from] ProfileError),

	#[error("Browser error.")]
	Browser(#[from] BrowserError),

	#[error("Install error.")]
	Install(#[from] InstallError),

	#[error("Query error.")]
	Query(#[from] QueryError),
}

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
	#[error("Browser error.")]
	Browser(#[from] BrowserError),

	#[error("Cannot create folder.")]
	Folder(#[from] std::io::Error),

	#[error("Ini error.")]
	Ini(#[from] ini::Error),

	#[error("Profile not found.")]
	ProfileNotFound,
}

#[derive(Debug, Error)]
pub enum QueryError {
	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("Request was not sent successfully.")]
	Send,

	#[error("Extension not found.")]
	ExtensionNotFound,
}
