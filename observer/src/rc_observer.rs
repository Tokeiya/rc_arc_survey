use crate::payload::Payload;
use crate::query_result::QueryResult;
use crate::report_data::ReportData;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

thread_local! {
static OBSERVERS: RefCell<HashMap<u64, Weak<Payload>>> = RefCell::new(HashMap::new());
}

//static OBSERVERS: std::thread::LocalKey<RefCell<HashMap<u64, Weak<Payload>>>>;

pub fn register() -> Rc<Payload> {
	let payload = Payload::generate();
	let id = payload.get();

	let rc = Rc::new(payload);
	OBSERVERS.with(|observers| {
		observers.borrow_mut().insert(id, Rc::downgrade(&rc));
	});
	rc
}

pub fn try_get_report(id: u64) -> QueryResult<ReportData> {
	fn get_report(rc: &Rc<Payload>) -> ReportData {
		let strong = Rc::strong_count(rc);
		let weak = Rc::weak_count(rc);
		ReportData::new(rc.get(), strong, weak)
	}

	match try_get(id) {
		QueryResult::Some(rc) => QueryResult::Some(get_report(&rc)),
		QueryResult::KeyNotFound => QueryResult::KeyNotFound,
		QueryResult::Dropped => QueryResult::Dropped,
		QueryResult::Locked => QueryResult::Locked,
	}
}

pub fn try_get(id: u64) -> QueryResult<Rc<Payload>> {
	OBSERVERS.with(|observers| {
		if let Some(elem) = observers.borrow().get(&id) {
			if let Some(rc) = elem.upgrade() {
				QueryResult::Some(rc)
			} else {
				QueryResult::Dropped
			}
		} else {
			QueryResult::KeyNotFound
		}
	})
}

pub fn vacuum() -> Vec<u64> {
	OBSERVERS.with(|observers| {
		let mut vec = Vec::new();

		for (id, _) in observers
			.borrow()
			.iter()
			.filter(|(_, y)| y.upgrade().is_none())
		{
			vec.push(*id);
		}

		for id in vec.iter() {
			observers.borrow_mut().remove(id);
		}

		vec
	})
}
