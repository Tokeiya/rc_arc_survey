#[derive(Debug, Clone, Copy)]
pub struct CountData {
	strong: usize,
	weak: usize,
}

impl CountData {
	pub(crate) fn new() -> Self {
		Self { strong: 0, weak: 0 }
	}

	pub fn strong(&self) -> usize {
		self.strong
	}
	pub fn weak(&self) -> usize {
		self.weak
	}
}
