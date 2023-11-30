use crate::args::Args;
use crate::errors::QueryError;
use crate::extension::Extension;
use crate::query;

use async_trait::async_trait;
use serde::Deserialize;

use super::Configurable;

#[derive(Debug, Clone, Deserialize)]
pub struct Extensions {
	pub extensions: Vec<Extension>,
}

#[async_trait]
impl Configurable for Extensions {
	type Err = QueryError;

	async fn configure_from(args: &Args) -> Result<Self, Self::Err> {
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
