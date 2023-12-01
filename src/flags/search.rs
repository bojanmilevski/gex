use crate::args::Args;
use crate::errors::QueryError;
use crate::extension::Extension;
use crate::flags::Configurable;
use crate::query;

use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Search {
	pub extensions: Vec<Extension>,
}

#[async_trait]
impl Configurable for Search {
	type Err = QueryError;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err> {
		if args.search.is_empty() {
			return Ok(Self { ..Default::default() });
		}

		let extensions = query::query_extensions(&args.search).await?;
		Ok(Self { extensions })
	}
}
