use crate::args::Args;
use crate::errors::Result;
use crate::extension::Extension;
use crate::flags::Configurable;
use crate::query;
use serde::Deserialize;

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
