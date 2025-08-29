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
}
