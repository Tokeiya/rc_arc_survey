use super::count_data::CountData;

#[derive(Debug)]
pub struct StateData {
	id: Option<usize>,
	count: Option<CountData>,
}

impl StateData {
	pub(crate) fn new(id: Option<usize>, count: Option<CountData>) -> Self {
		Self { id, count }
	}

	pub fn id(&self) -> Option<usize> {
		self.id
	}
	pub fn count(&self) -> Option<CountData> {
		self.count
	}
}
