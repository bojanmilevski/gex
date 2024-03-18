use crate::addon::addon::Addon;
use crate::errors::Result;

pub trait CRUD {
	fn create() -> Result<()>;
	fn read(&self) -> Result<()>;
	fn update(&self, addon: Addon) -> Result<()>;
	fn delete(&self, addon: Addon) -> Result<()>;
}
