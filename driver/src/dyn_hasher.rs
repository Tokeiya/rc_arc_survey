use std::hash::Hasher;

pub struct DynHasher(Box<dyn Hasher>);

impl From<Box<dyn Hasher>> for DynHasher {
	fn from(value: Box<dyn Hasher>) -> Self {
		Self(value)
	}
}
impl Hasher for DynHasher {
	fn finish(&self) -> u64 {
		self.0.finish()
	}

	fn write(&mut self, bytes: &[u8]) {
		self.0.write(bytes)
	}
}
