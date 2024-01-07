use super::configurable::Configurable;
use crate::cli::Cli;

pub struct List {}

impl Configurable for List {
	async fn try_configure_from(cli: &Cli) -> crate::errors::Result<Self> {
		Ok(Self {})
	}
}
