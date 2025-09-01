use std::any::Any;
use std::hash::{Hash, Hasher};

pub trait DynKey {
	fn as_any(&self) -> &dyn Any;
	fn dyn_eq<T: DynKey>(&self, other: &T) -> bool;
	fn dyn_hash<H: Hasher>(&self, state: &mut H);
}

impl<T> DynKey for T
where
	T: Any + Hash + PartialEq,
{
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn dyn_eq<U: DynKey>(&self, other: &U) -> bool {
		if let Some(o) = other.as_any().downcast_ref::<T>() {
			self == o
		} else {
			false
		}
	}

	fn dyn_hash<H: Hasher>(&self, state: &mut H) {
		self.hash(state);
	}
}
