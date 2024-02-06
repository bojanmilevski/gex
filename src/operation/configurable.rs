use crate::cli::Cli;
use crate::configuration::profile::Profile;
use crate::errors::Result;

pub trait Configurable: Sized {
	async fn try_configure_from(cli: Cli, profile: Profile) -> Result<Self>;
}
