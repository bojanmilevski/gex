use super::flags::Flags;
use crate::errors::Result;

pub trait Runnable {
	async fn try_run(&self, flags: &Flags) -> Result<()>;
}
