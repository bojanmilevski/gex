use crate::cli::cli::Cli;
use anyhow::Result;

// FIX:
pub trait Configurable: Sized {
	async fn try_configure_from(cli: Cli) -> Result<Self>;
}
