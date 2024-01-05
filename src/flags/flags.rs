use crate::args::Args;
use crate::errors::Result;
use crate::flags::configurable::Configurable;
use crate::flags::extensions::Extensions;
use crate::flags::profile::Profile;
use crate::flags::search::Search;
use std::fmt::Display;

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

impl Display for Flags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Extensions: {}\nProfile: {}\nBrowser: {}\nSearch: {}\n",
			self.extensions, self.profile, self.profile.browser, self.search
		)
	}
}

impl Into<String> for Flags {
	fn into(self) -> String {
		String::from(format!(
			"Extensions: {}\nProfile: {}\nBrowser: {}\nSearch: {}\n",
			self.extensions, self.profile, self.profile.browser, self.search
		))
	}
}
