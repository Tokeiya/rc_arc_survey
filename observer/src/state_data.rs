use crate::dummy_load::DummyLoad;
use std::rc::Rc;
use std::sync::{Arc, Weak as SyncWeak};
#[derive(Debug)]
pub struct StateData {
	id: Option<usize>,
	strong: usize,
	weak: usize,
}

impl StateData {
	pub fn id(&self) -> Option<usize> {
		self.id
	}

	pub fn strong(&self) -> usize {
		self.strong
	}
	pub fn weak(&self) -> usize {
		self.weak
	}
}

impl From<&Arc<DummyLoad>> for StateData {
	fn from(value: &Arc<DummyLoad>) -> Self {
		Self {
			id: Some(value.id()),
			strong: Arc::strong_count(value),
			weak: Arc::weak_count(value),
		}
	}
}

impl From<&SyncWeak<DummyLoad>> for StateData {
	fn from(value: &SyncWeak<DummyLoad>) -> Self {
		if let Some(arc) = value.upgrade() {
			Self {
				id: Some(arc.id()),
				strong: Arc::strong_count(&arc) - 1,
				weak: Arc::weak_count(&arc) - 1,
			}
		} else {
			Self {
				id: None,
				strong: 0,
				weak: Arc::weak_count(value),
			}
		}
	}
}

impl From<&Rc<DummyLoad>> for StateData {
	fn from(value: &Rc<DummyLoad>) -> Self {
		todo!()
	}
}
