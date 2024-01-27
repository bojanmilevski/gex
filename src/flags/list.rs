use super::configurable::Configurable;
use crate::cli::Cli;
use crate::errors::Result;

pub struct List {}

impl Configurable for List {
	async fn try_configure_from(cli: &Cli) -> Result<Self> {
		Ok(Self {})
	}
}
