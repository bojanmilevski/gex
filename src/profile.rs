#[derive(Debug, Default)]
pub struct Profile {
	pub name: String,
	pub path: String,
}

impl Profile {
	pub fn from(name: String, path: String) -> Self {
		Self { name, path }
	}
}
