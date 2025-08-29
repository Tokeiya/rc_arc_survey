#[derive(Debug, Copy, Clone)]
pub struct CountData {
	strong: usize,
	weak: usize,
}

impl CountData {
	pub(crate) fn new(strong: usize, weak: usize) -> Self {
		Self { strong: 0, weak: 0 }
	}

	pub fn strong(&self) -> usize {
		self.strong
	}
	pub fn weak(&self) -> usize {
		self.weak
	}
}
