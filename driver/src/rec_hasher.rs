use std::hash::Hasher;

pub struct RecHasher(Vec<u8>);

impl RecHasher {
	pub fn new() -> RecHasher {
		RecHasher(Vec::new())
	}

	pub fn replay(&self) -> Vec<u8> {
		self.0.clone()
	}
}

impl Hasher for RecHasher {
	fn finish(&self) -> u64 {
		panic!("Not supported")
	}

	fn write(&mut self, bytes: &[u8]) {
		for elem in bytes {
			self.0.push(*elem);
		}
	}
}
