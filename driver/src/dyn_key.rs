use crate::dyn_hasher::DynHasher;
use std::any::Any;
use std::hash::Hash;

pub trait DynKey {
	fn as_any(&self) -> &dyn Any;
	fn dyn_eq(&self, other: &dyn DynKey) -> bool;
	fn dyn_hash(&self, state: &mut DynHasher);
}

impl<T> DynKey for T
where
	T: Any + Hash + PartialEq,
{
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn dyn_eq(&self, other: &dyn DynKey) -> bool {
		if let Some(other) = other.as_any().downcast_ref::<T>() {
			self == other
		} else {
			false
		}
	}

	fn dyn_hash(&self, state: &mut DynHasher) {
		self.hash(state);
	}
}
