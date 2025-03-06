use crate::cli::Cli;
use anyhow::Result;

// FIX:
pub trait Initializable: Sized {
	async fn try_init(cli: Cli) -> Result<Self>;
}

// FIX: should not be mut
pub trait Runnable {
	async fn try_run(&mut self) -> Result<()>;
}
