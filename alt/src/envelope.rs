use std::hash::Hasher;

pub struct Envelope<'a>(&'a mut dyn Hasher);

impl<'a> From<&'a mut dyn Hasher> for Envelope<'a> {
	fn from(value: &'a mut dyn Hasher) -> Self {
		Self(value)
	}
}

impl<'a> Hasher for Envelope<'a> {
	fn finish(&self) -> u64 {
		self.0.finish()
	}

	fn write(&mut self, bytes: &[u8]) {
		self.0.write(bytes);
	}
}
