use anyhow::Result;

// FIX: should not be mut?
pub trait Runnable {
	async fn try_run(&mut self) -> Result<()>;
}
