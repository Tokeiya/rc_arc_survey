#[derive(Debug)]
pub struct ReportData {
	id: u64,
	strong: usize,
	weak: usize,
}

impl ReportData {
	pub fn new(id: u64, strong: usize, weak: usize) -> Self {
		Self { id, strong, weak }
	}

	pub fn get_id(&self) -> u64 {
		self.id
	}

	pub fn get_strong(&self) -> usize {
		self.strong
	}

	pub fn get_weak(&self) -> usize {
		self.weak
	}
}
