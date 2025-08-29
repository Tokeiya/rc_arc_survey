use crate::dummy_load::{generate_dummy, DummyLoad};
use crate::state_data::StateData;
use dashmap::DashMap;
use std::sync::{Arc, LazyLock, Weak};

static OBSERVERS: LazyLock<DashMap<usize, Weak<DummyLoad>>> = LazyLock::new(DashMap::new);

pub fn yield_and_register() -> Arc<DummyLoad> {
	let dummy = generate_dummy();
	let id = dummy.id();

	let arc = Arc::new(dummy);
	OBSERVERS.insert(id, Arc::downgrade(&arc));

	arc
}

pub fn vacuum() -> Vec<usize> {
	let mut ret = Vec::new();

	for elem in OBSERVERS
		.iter()
		.filter(|item| item.value().upgrade().is_none())
	{
		ret.push(*elem.key());
	}

	for id in ret.iter() {
		OBSERVERS.remove(id);
	}
	ret
}

pub fn iter() -> impl Iterator<Item = (usize, Option<Arc<DummyLoad>>)> {
	OBSERVERS
		.iter()
		.map(|item| (*item.key(), item.value().upgrade()))
}

pub fn try_get(id: usize) -> Option<Arc<DummyLoad>> {
	todo!()
}

pub fn try_get_report(id: usize) -> Option<StateData> {
	todo!()
}
