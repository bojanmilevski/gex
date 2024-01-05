use crate::args::Args;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::flags::configurable::Configurable;
use crate::query;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Search {
	pub extensions: Vec<Extension>,
}

impl Configurable for Search {
	async fn configure_from(args: &Args) -> Result<Self> {
		if args.search.is_none() {
			return Ok(Self { ..Default::default() });
		}

		let extensions = query::query_extensions(&args.search.clone().unwrap()).await?;
		Ok(Self { extensions })
	}
}

impl Display for Search {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "TODO: impl Display for Search")
	}
}

impl Into<String> for Search {
	fn into(self) -> String {
		String::from("TODO: impl Into<String> for Search")
	}
}
