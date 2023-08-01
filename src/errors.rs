use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfileError {
	#[error("Can not find $HOME enviornment variable.")]
	HomeVar(#[from] std::env::VarError),

	#[error("Ini error.")]
	Ini(#[from] ini::Error),
}

#[derive(Debug, Error)]
pub enum InstallError {
	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("Profile path error.")]
	Profile(#[from] ProfileError),

	#[error("Creating file error.")]
	CreateFile(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ArgsError {
	#[error("Profile error.")]
	Profile(#[from] ProfileError),

	#[error("Specified profile does not exist.")]
	ProfileNotFound,
}
