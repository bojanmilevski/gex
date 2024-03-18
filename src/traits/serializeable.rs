use crate::errors::Result;

pub trait Serializable {
	fn serialize(&self) -> Result<()>;
}
