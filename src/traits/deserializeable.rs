use crate::errors::Result;

pub trait Deserializable {
	fn deserialize(&self) -> Result<()>;
}
