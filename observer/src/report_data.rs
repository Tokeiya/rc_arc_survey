use super::count_data::CountData;

#[derive(Debug)]
pub struct ReportData {
	id: Option<usize>,
	count: Option<CountData>,
}

impl ReportData {
	pub fn new(id: Option<usize>, count: Option<CountData>) -> Self {
		Self { id, count }
	}

	pub fn from_id(id: usize) -> Self {
		Self {
			id: Some(id),
			count: None,
		}
	}
	pub fn id(&self) -> Option<usize> {
		self.id
	}
	pub fn count(&self) -> Option<CountData> {
		self.count
	}
}
