use crate::cli::Cli;
use crate::errors::Result;

pub trait Configurable: Sized {
	async fn try_configure_from(cli: &Cli) -> Result<Self>;
}
