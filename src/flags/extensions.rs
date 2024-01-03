use crate::args::Args;
use crate::errors::Result;
use crate::extension::Extension;
use crate::flags::Configurable;
use crate::query;
use serde::Deserialize;

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
