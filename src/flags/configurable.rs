use crate::args::Args;
use crate::errors::Result;
use std::fmt::Display;

pub trait Configurable: Sized + Display + Into<String> {
	async fn configure_from(args: &Args) -> Result<Self>;
}
