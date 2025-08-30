use crate::payload::Payload;
use crate::query_result::QueryResult;
use crate::report_data::ReportData;
use dashmap::try_result::TryResult;
use dashmap::DashMap;
use std::sync::Weak;
use std::sync::{Arc, LazyLock};

static OBSERVERS: LazyLock<DashMap<u64, Weak<Payload>>> = LazyLock::new(|| DashMap::new());

pub fn register() -> Arc<Payload> {
	let payload = Payload::generate();
	let id = payload.get();

	let arc = Arc::new(payload);

	OBSERVERS.insert(id, Arc::downgrade(&arc));
	arc
}

pub fn try_get_report(id: u64) -> QueryResult<ReportData> {
	fn get_report(arc: &Arc<Payload>) -> ReportData {
		let strong = Arc::strong_count(arc);
		let weak = Arc::weak_count(arc);
		ReportData::new(arc.get(), strong, weak)
	}

	match try_get(id) {
		QueryResult::Some(arc) => QueryResult::Some(get_report(&arc)),
		QueryResult::KeyNotFound => QueryResult::KeyNotFound,
		QueryResult::Dropped => QueryResult::Dropped,
		QueryResult::Locked => QueryResult::Locked,
	}
}

pub fn try_get(id: u64) -> QueryResult<Arc<Payload>> {
	match OBSERVERS.try_get(&id) {
		TryResult::Present(weak) => {
			if let Some(arc) = weak.upgrade() {
				QueryResult::Some(arc)
			} else {
				QueryResult::Dropped
			}
		}
		TryResult::Absent => QueryResult::KeyNotFound,
		TryResult::Locked => QueryResult::Locked,
	}
}

pub fn vacuum() -> Vec<u64> {
	todo!()
}
