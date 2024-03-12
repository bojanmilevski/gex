use crate::errors::Result;

pub trait Runnable {
	async fn try_run(&self) -> Result<()>;
}
