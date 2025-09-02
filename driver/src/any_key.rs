use crate::rec_hassher::RecHasher;
use std::any::Any;
use std::hash::Hash;

pub trait AnyKey {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn equal(&self, other: &dyn AnyKey) -> bool;

	fn prepare_hash(&self) -> Vec<u8>;
}

impl<T> AnyKey for T
where
	T: Any + Hash + PartialEq + Eq,
{
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}

	fn equal(&self, other: &dyn AnyKey) -> bool {
		let tmp = other.as_any();

		if let Some(other) = tmp.downcast_ref::<T>() {
			self == other
		} else {
			false
		}
	}

	fn prepare_hash(&self) -> Vec<u8> {
		let mut hasher = RecHasher::new();
		self.hash(&mut hasher);

		hasher.replay()
	}
}
