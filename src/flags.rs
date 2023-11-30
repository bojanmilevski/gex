pub mod browser;
pub mod extensions;
pub mod profile;
pub mod search;

pub use browser::Browser;
pub use extensions::Extensions;
pub use profile::Profile;
pub use search::Search;

use crate::args::Args;
use crate::errors::FlagsError;

use async_trait::async_trait;

#[async_trait]
pub trait Configurable: Sized {
	type Err;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err>;
}

pub struct Flags {
	pub profile: Profile,
	pub browser: Browser,
	pub extensions: Extensions,
	pub search: Search,
}

#[async_trait]
impl Configurable for Flags {
	type Err = FlagsError;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err> {
		Ok(Self {
			profile: Profile::configure_from(&args).await?,
			browser: Browser::configure_from(&args).await?,
			extensions: Extensions::configure_from(&args).await?,
			search: Search::configure_from(&args).await?,
		})
	}
}
