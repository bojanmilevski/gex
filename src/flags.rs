pub mod browser;
pub mod extensions;
pub mod profile;
pub mod search;

use crate::args::Args;
use crate::errors::Result;
pub use browser::Browser;
pub use extensions::Extensions;
pub use profile::Profile;
pub use search::Search;

pub trait Configurable: Sized {
	async fn configure_from(args: &Args) -> Result<Self>;
}

pub struct Flags {
	pub extensions: Extensions,
	pub profile: Profile,
	pub search: Search,
}

impl Configurable for Flags {
	async fn configure_from(args: &Args) -> Result<Self> {
		Ok(Self {
			extensions: Extensions::configure_from(&args).await?,
			profile: Profile::configure_from(&args).await?,
			search: Search::configure_from(&args).await?,
		})
	}
}
