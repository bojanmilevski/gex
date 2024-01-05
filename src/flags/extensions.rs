use crate::args::Args;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::flags::configurable::Configurable;
use crate::query;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Extensions {
	pub extensions: Vec<Extension>,
}

impl Configurable for Extensions {
	async fn configure_from(args: &Args) -> Result<Self> {
		if args.extensions.is_empty() {
			return Ok(Self { ..Default::default() });
		}

		let mut extensions = Vec::new();

		for extension in &args.extensions {
			match query::query_extension(&extension).await {
				Ok(ext) => extensions.push(ext),
				Err(err) => return Err(err),
			};
		}

		Ok(Self { extensions })
	}
}

impl Display for Extensions {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "TODO: impl Display for Extensions")
	}
}

impl Into<String> for Extensions {
	fn into(self) -> String {
		String::from("TODO: impl Into<String> for Extensions")
	}
}
