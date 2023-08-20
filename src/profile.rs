#[derive(Debug, Default, Clone)]
pub struct Profile {
	pub name: String,
	pub path: String,
}

impl Profile {
	pub fn new(name: String, path: String) -> Self {
		Self { name, path }
	}
}
