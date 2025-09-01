use crate::dyn_key::DynKey;
use std::hash::{Hash, Hasher};

pub struct Key(Box<dyn DynKey>);

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		self.0.dyn_eq(other)
	}
}

impl Hash for Key {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}
